use bevy::ecs::{
    component::ComponentId,
    world::{EntityWorldMut, FilteredEntityMut},
};
use bevy_mod_ffi_core::{entity_world_mut, filtered_entity_mut};

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
