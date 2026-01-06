use crate::{SharedRegistry, SharedSystemState};
use bevy::{
    ecs::{
        entity::Entity, event::Event, observer::On, prelude::*, system::DynSystemParam,
        world::World,
    },
    prelude::*,
};
use bevy_mod_ffi_core::{dyn_system_param, system_state, world, RunObserverFn};
use std::{any::Any, ffi::CStr, marker::PhantomData, slice, sync::Arc};

#[derive(Clone)]
pub struct LibraryHandle(pub Arc<dyn Any + Send + Sync>);
#[derive(Event, Clone, Copy)]
pub struct EntityEventWrapper<E> {
    pub entity: Entity,
    pub inner: E,
}

impl<E> bevy::ecs::event::EntityEvent for EntityEventWrapper<E>
where
    E: Event + Clone + Copy,
    for<'a> E::Trigger<'a>: Default,
{
    fn event_target(&self) -> Entity {
        self.entity
    }

    fn event_target_mut(&mut self) -> &mut Entity {
        &mut self.entity
    }
}

pub trait Observable: Send + Sync + 'static {
    fn type_path(&self) -> &'static str;

    fn add_observer_with_state(
        &self,
        world: &mut World,
        state: Box<SharedSystemState>,
        f_ptr: usize,
        run_observer_fn: RunObserverFn,
        library_handle: Option<LibraryHandle>,
    ) -> Entity;

    fn add_entity_observer_with_state(
        &self,
        world: &mut World,
        entity: Entity,
        state: Box<SharedSystemState>,
        f_ptr: usize,
        run_observer_fn: RunObserverFn,
        library_handle: Option<LibraryHandle>,
    ) -> Entity;

    fn trigger(&self, world: &mut World, event_data: &[u8]);

    fn trigger_for_entity(&self, world: &mut World, event_data: &[u8], entity: Entity);
}

pub struct ObservableOf<E> {
    _marker: PhantomData<E>,
}

impl<E> ObservableOf<E> {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<E> Default for ObservableOf<E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<E: Event + Clone + Copy + bevy::reflect::TypePath> Observable for ObservableOf<E>
where
    for<'a> E::Trigger<'a>: Default,
{
    fn type_path(&self) -> &'static str {
        E::type_path()
    }

    fn add_observer_with_state(
        &self,
        world: &mut World,
        state: Box<SharedSystemState>,
        f_ptr: usize,
        run_observer_fn: RunObserverFn,
        library_handle: Option<LibraryHandle>,
    ) -> Entity {
        let observer_system =
            state.build_any_system(move |on: On<E>, params: Vec<DynSystemParam>| {
                let _ = &library_handle;

                let mut param_ptrs: Vec<*mut dyn_system_param> = Vec::new();
                for param in params {
                    let boxed = Box::new(param);
                    param_ptrs.push(Box::into_raw(boxed) as *mut dyn_system_param);
                }
                let len = param_ptrs.len();
                let pointers_ptr = param_ptrs.as_ptr();

                let event_ptr = on.event() as *const E as *const u8;

                unsafe {
                    run_observer_fn(f_ptr as _, pointers_ptr, len, event_ptr as _);
                };
            });

        world.add_observer(observer_system).id()
    }

    fn trigger(&self, world: &mut World, event_data: &[u8]) {
        let event = unsafe { *(event_data.as_ptr() as *const E) };
        world.trigger(event);
    }

    fn add_entity_observer_with_state(
        &self,
        world: &mut World,
        entity: Entity,
        state: Box<SharedSystemState>,
        f_ptr: usize,
        run_observer_fn: RunObserverFn,
        library_handle: Option<LibraryHandle>,
    ) -> Entity {
        let observer_system = state.build_any_system(
            move |on: On<EntityEventWrapper<E>>, params: Vec<DynSystemParam>| {
                let _ = &library_handle;

                let mut param_ptrs: Vec<*mut dyn_system_param> = Vec::new();
                for param in params {
                    let boxed = Box::new(param);
                    param_ptrs.push(Box::into_raw(boxed) as *mut dyn_system_param);
                }
                let len = param_ptrs.len();
                let pointers_ptr = param_ptrs.as_ptr();

                let event_ptr = &on.event().inner as *const E as *const u8;

                unsafe {
                    run_observer_fn(f_ptr as _, pointers_ptr, len, event_ptr as _);
                };
            },
        );

        world.entity_mut(entity).observe(observer_system).id()
    }

    fn trigger_for_entity(&self, world: &mut World, event_data: &[u8], entity: Entity) {
        let inner = unsafe { *(event_data.as_ptr() as *const E) };
        let wrapped = EntityEventWrapper { entity, inner };
        world.trigger(wrapped);
    }
}

#[derive(Resource, Clone)]
pub struct CurrentLibraryHandle(pub Option<LibraryHandle>);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_system_state_build_on(
    world_ptr: *mut world,
    state_ptr: *mut system_state,
    event_name_ptr: *const u8,
    event_name_len: usize,
    f_ptr: *mut (),
    run_observer_fn: RunObserverFn,
) -> bool {
    let world = unsafe { &mut *(world_ptr as *mut World) };
    let state: Box<SharedSystemState> = unsafe { Box::from_raw(state_ptr as _) };

    let event_name_bytes = unsafe { slice::from_raw_parts(event_name_ptr, event_name_len) };
    let event_name = match CStr::from_bytes_with_nul(event_name_bytes) {
        Ok(cstr) => match cstr.to_str() {
            Ok(s) => s,
            Err(_) => return false,
        },
        Err(_) => return false,
    };

    let f_ptr_n = f_ptr as usize;

    let library_handle = world
        .get_resource::<CurrentLibraryHandle>()
        .and_then(|h| h.0.clone());

    let mut registry = match world.remove_resource::<SharedRegistry>() {
        Some(r) => r,
        None => return false,
    };

    if let Some(event_ops) = registry.events.remove(event_name) {
        let observer_entity = event_ops.add_observer_with_state(
            world,
            state,
            f_ptr_n,
            run_observer_fn,
            library_handle,
        );

        registry.register_observer(observer_entity);

        let key = event_ops.type_path();
        registry.events.insert(key, event_ops);
        world.insert_resource(registry);
        true
    } else {
        world.insert_resource(registry);
        false
    }
}
