use crate::{LoadedLibrary, SharedRegistry};
use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    ecs::world::World,
    platform::collections::HashMap,
    prelude::*,
};
use bevy_mod_ffi_host_sys::{CurrentLibraryHandle, LibraryHandle};
use libloading::{Library, Symbol};
use std::{path::PathBuf, sync::Arc};
use thiserror::Error;

#[derive(Asset, TypePath, Debug)]
pub struct DynamicPlugin {
    pub path: PathBuf,
}

#[derive(Debug, Error)]
pub enum DynamicPluginLoaderError {
    #[error("Failed to get asset path")]
    NoPath,
}

#[derive(Default)]
pub struct DynamicPluiginLoader;

impl AssetLoader for DynamicPluiginLoader {
    type Asset = DynamicPlugin;
    type Settings = ();
    type Error = DynamicPluginLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        settings: &Self::Settings,
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let _ = reader;
        let _ = settings;

        let path = load_context.path().to_path_buf();
        Ok(DynamicPlugin { path })
    }

    fn extensions(&self) -> &[&str] {
        &[]
    }
}

#[derive(Component)]
pub struct DynamicPluginHandle {
    handle: Handle<DynamicPlugin>,
    loaded: Option<LoadedLibrary>,
}

#[derive(Resource, Default)]
pub struct DynamicPlugins {
    asset_to_entity: HashMap<AssetId<DynamicPlugin>, Entity>,
}

pub struct FfiPlugin;

impl Plugin for FfiPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<DynamicPlugin>()
            .init_asset_loader::<DynamicPluiginLoader>()
            .init_resource::<DynamicPlugins>()
            .add_systems(Update, handle_library_loading)
            .add_systems(Update, handle_library_changes);
    }
}

fn handle_library_loading(
    asset_server: Res<AssetServer>,
    libraries: Res<Assets<DynamicPlugin>>,
    mut tracker: ResMut<DynamicPlugins>,
    mut query: Query<(Entity, &mut DynamicPluginHandle)>,
    mut commands: Commands,
) {
    for (entity, handle) in query.iter_mut() {
        if handle.loaded.is_some() {
            continue;
        }

        let asset_id = handle.handle.id();

        if libraries.get(asset_id).is_some() {
            let Some(asset_path) = asset_server.get_path(asset_id) else {
                continue;
            };

            tracker.asset_to_entity.insert(asset_id, entity);
            let path = asset_path.path().to_path_buf();
            commands.queue(move |world: &mut World| {
                match unsafe { load_library_from_path_world(&path, world) } {
                    Ok(loaded) => {
                        info!("Loaded guest library: {:?}", path);
                        if let Ok(mut entity_mut) = world.get_entity_mut(entity) {
                            if let Some(mut lib_handle) =
                                entity_mut.get_mut::<DynamicPluginHandle>()
                            {
                                lib_handle.loaded = Some(loaded);
                            }
                        }
                    }
                    Err(e) => {
                        error!("Failed to load guest library {:?}: {}", path, e);
                    }
                }
            });
        }
    }
}

fn handle_library_changes(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    libraries: Res<Assets<DynamicPlugin>>,
    mut events: MessageReader<AssetEvent<DynamicPlugin>>,
    mut tracker: ResMut<DynamicPlugins>,
    mut query: Query<(Entity, &mut DynamicPluginHandle)>,
) {
    for event in events.read() {
        match event {
            AssetEvent::Modified { id } => {
                if let Some(&entity) = tracker.asset_to_entity.get(id) {
                    if let Ok((_, mut handle)) = query.get_mut(entity) {
                        if let Some(loaded) = handle.loaded.take() {
                            let loaded_clone = loaded.clone();
                            commands.queue(move |world: &mut World| {
                                loaded_clone.unload(world);
                            });
                        }

                        if libraries.get(*id).is_some() {
                            let Some(asset_path) = asset_server.get_path(*id) else {
                                continue;
                            };
                            let path = asset_path.path().to_path_buf();

                            commands.queue(move |world: &mut World| {
                                match unsafe { load_library_from_path_world(&path, world) } {
                                    Ok(loaded) => {
                                        if let Ok(mut entity_mut) = world.get_entity_mut(entity) {
                                            if let Some(mut lib_handle) =
                                                entity_mut.get_mut::<DynamicPluginHandle>()
                                            {
                                                lib_handle.loaded = Some(loaded);
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        error!(
                                            "Failed to hot-reload guest library {:?}: {}",
                                            path, e
                                        );
                                    }
                                }
                            });
                        }
                    }
                }
            }
            AssetEvent::Removed { id } => {
                if let Some(entity) = tracker.asset_to_entity.remove(id) {
                    if let Ok((_, mut handle)) = query.get_mut(entity) {
                        if let Some(loaded) = handle.loaded.take() {
                            commands.queue(move |world: &mut World| {
                                loaded.unload(world);
                            });
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

pub(crate) unsafe fn load_library_from_path_world(
    path: &std::path::Path,
    world: &mut World,
) -> Result<LoadedLibrary, Box<dyn std::error::Error + Send + Sync>> {
    let guest_lib = Arc::new(unsafe { Library::new(path)? });

    let library_id = {
        let mut registry = world
            .remove_resource::<SharedRegistry>()
            .ok_or("SharedRegistry resource not found")?;
        let id = registry.new_library_id();
        registry.set_current_library(Some(id));
        world.insert_resource(registry);
        id
    };

    let library_handle = LibraryHandle(guest_lib.clone());
    world.insert_resource(CurrentLibraryHandle(Some(library_handle)));

    let main_fn: Symbol<unsafe extern "C" fn(*mut bevy_mod_ffi_core::world)> =
        unsafe { guest_lib.get(b"bevy_main")? };
    unsafe { main_fn(world as *mut World as *mut bevy_mod_ffi_core::world) };

    if let Some(mut registry) = world.remove_resource::<SharedRegistry>() {
        registry.set_current_library(None);
        world.insert_resource(registry);
    }
    world.remove_resource::<CurrentLibraryHandle>();

    Ok(LoadedLibrary::new(guest_lib, library_id))
}
