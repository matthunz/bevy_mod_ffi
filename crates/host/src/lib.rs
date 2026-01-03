#![allow(clippy::missing_safety_doc)]

pub use bevy_mod_ffi_host_sys as sys;

use bevy::ecs::world::World;
use libloading::{Library, Symbol};
use std::{error::Error, ffi::OsStr, sync::Arc};

pub mod query;

pub mod system;

pub mod world;

use bevy_mod_ffi_host_sys::{CurrentLibraryHandle, LibraryHandle};
pub use bevy_mod_ffi_host_sys::{LibraryId, SharedRegistry};
#[derive(Clone)]
pub struct LoadedLibrary {
    id: LibraryId,
    _library: Arc<Library>,
}

impl LoadedLibrary {
    pub fn unload(self, world: &mut World) {
        world.remove_resource::<CurrentLibraryHandle>();

        if let Some(mut registry) = world.remove_resource::<SharedRegistry>() {
            if let Some(observers) = registry.take_library_observers(self.id) {
                world.insert_resource(registry);

                for observer in observers {
                    if world.get_entity(observer).is_ok() {
                        world.despawn(observer);
                    }
                }
            } else {
                world.insert_resource(registry);
            }
        }
    }
}
pub unsafe fn run(
    path: impl AsRef<OsStr>,
    world: &mut World,
) -> Result<LoadedLibrary, Box<dyn Error>> {
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

    {
        if let Some(mut registry) = world.remove_resource::<SharedRegistry>() {
            registry.set_current_library(None);
            world.insert_resource(registry);
        }
    }
    world.remove_resource::<CurrentLibraryHandle>();

    Ok(LoadedLibrary {
        _library: guest_lib,
        id: library_id,
    })
}
