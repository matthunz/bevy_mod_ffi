use crate::{system::observer::CurrentLibraryHandle, SharedRegistry};
use bevy::ecs::{
    component::ComponentId,
    system::SystemState,
    world::{EntityWorldMut, FilteredEntityMut},
};
use bevy_mod_ffi_core::{entity_world_mut, filtered_entity_mut, system_state, RunObserverFn};
use std::{ffi::CStr, slice};

type SharedSystemState = SystemState<(Vec<bevy::ecs::system::DynSystemParam<'static, 'static>>,)>;

type SharedEntityRef = FilteredEntityMut<'static, 'static>;

#[unsafe(no_mangle)]
unsafe extern "C" fn bevy_world_entity_mut_drop(entity_ptr: *mut entity_world_mut) {
    let _ = unsafe { Box::from_raw(entity_ptr as *mut EntityWorldMut) };
}

#[unsafe(no_mangle)]
unsafe extern "C" fn bevy_filtered_entity_mut_get_component(
    entity_ptr: *mut filtered_entity_mut,
    component_id: usize,
    out_ptr: *mut *mut u8,
) -> bool {
    let shared_entity = unsafe { &mut *(entity_ptr as *mut SharedEntityRef) };

    let bevy_component_id = ComponentId::new(component_id);
    let ptr = match shared_entity.get_by_id(bevy_component_id) {
        Some(p) => p,
        None => return false,
    };

    unsafe {
        *out_ptr = ptr.as_ptr() as _;
    }

    true
}

#[unsafe(no_mangle)]
unsafe extern "C" fn bevy_filtered_entity_mut_get_component_mut(
    entity_ptr: *mut filtered_entity_mut,
    component_id: usize,
    out_ptr: *mut *mut u8,
) -> bool {
    let shared_entity = unsafe { &mut *(entity_ptr as *mut SharedEntityRef) };

    let bevy_component_id = ComponentId::new(component_id);
    let ptr = match shared_entity.get_mut_by_id(bevy_component_id) {
        Some(p) => p,
        None => return false,
    };

    unsafe {
        *out_ptr = ptr.into_inner().as_ptr() as _;
    }

    true
}

#[unsafe(no_mangle)]
unsafe extern "C" fn bevy_filtered_entity_mut_drop(entity_ptr: *mut filtered_entity_mut) {
    let _ = unsafe { Box::from_raw(entity_ptr as *mut SharedEntityRef) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_entity_world_mut_observe(
    entity_ptr: *mut entity_world_mut,
    state_ptr: *mut system_state,
    event_name_ptr: *const u8,
    event_name_len: usize,
    f_ptr: *mut (),
    run_observer_fn: RunObserverFn,
) -> bool {
    let entity_world_mut = unsafe { &mut *(entity_ptr as *mut EntityWorldMut) };
    let entity = entity_world_mut.id();
    let world = entity_world_mut.world_mut();
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
        let observer_entity = event_ops.add_entity_observer_with_state(
            world,
            entity,
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

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_entity_world_mut_trigger(
    entity_ptr: *mut entity_world_mut,
    event_name_ptr: *const u8,
    event_name_len: usize,
    event_data_ptr: *const u8,
    event_data_len: usize,
) -> bool {
    let entity_world_mut = unsafe { &mut *(entity_ptr as *mut EntityWorldMut) };
    let entity = entity_world_mut.id();
    let world = entity_world_mut.world_mut();

    let event_name_bytes = unsafe { slice::from_raw_parts(event_name_ptr, event_name_len) };
    let event_name = match CStr::from_bytes_with_nul(event_name_bytes) {
        Ok(cstr) => match cstr.to_str() {
            Ok(s) => s,
            Err(_) => return false,
        },
        Err(_) => return false,
    };

    let event_data = unsafe { slice::from_raw_parts(event_data_ptr, event_data_len) };

    let mut registry = match world.remove_resource::<SharedRegistry>() {
        Some(r) => r,
        None => return false,
    };

    if let Some(event_ops) = registry.events.remove(event_name) {
        event_ops.trigger_for_entity(world, event_data, entity);
        let key = event_ops.type_path();
        registry.events.insert(key, event_ops);
        world.insert_resource(registry);
        true
    } else {
        world.insert_resource(registry);
        false
    }
}
