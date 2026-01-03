use bevy::ecs::{
    component::ComponentId,
    entity::Entity,
    query::QueryState,
    world::{DeferredWorld, FilteredEntityMut},
};
use bevy_mod_ffi_core::{deferred_world, query, query_state};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_deferred_world_query(
    deferred_ptr: *mut deferred_world,
    query_state_ptr: *mut query_state,
    out_query: *mut *mut query,
) -> bool {
    let deferred = unsafe { &mut *(deferred_ptr as *mut DeferredWorld) };
    let query_state =
        unsafe { &mut *(query_state_ptr as *mut QueryState<FilteredEntityMut<'static, 'static>>) };

    let query = deferred.query(query_state);
    unsafe {
        *out_query = Box::into_raw(Box::new(query)) as *mut query;
    }

    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_deferred_world_get_mut(
    deferred_ptr: *mut deferred_world,
    entity_bits: u64,
    component_id: usize,
    out_ptr: *mut *mut u8,
) -> bool {
    let deferred = unsafe { &mut *(deferred_ptr as *mut DeferredWorld) };
    let entity = Entity::from_bits(entity_bits);
    let component_id = ComponentId::new(component_id);

    let Some(mut mut_untyped) = deferred.get_mut_by_id(entity, component_id) else {
        return false;
    };

    unsafe {
        *out_ptr = mut_untyped.as_mut().as_ptr();
    }

    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_deferred_world_get_resource_mut(
    deferred_ptr: *mut deferred_world,
    component_id: usize,
    out_ptr: *mut *mut u8,
) -> bool {
    let deferred = unsafe { &mut *(deferred_ptr as *mut DeferredWorld) };
    let component_id = ComponentId::new(component_id);

    let Some(mut mut_untyped) = deferred.get_resource_mut_by_id(component_id) else {
        return false;
    };

    unsafe {
        *out_ptr = mut_untyped.as_mut().as_ptr();
    }

    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_deferred_world_drop(deferred_ptr: *mut deferred_world) {
    let _ = unsafe { Box::from_raw(deferred_ptr as *mut DeferredWorld) };
}
