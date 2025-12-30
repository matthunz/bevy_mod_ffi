use bevy::{ecs::world::World, prelude::*};
use bevy_mod_ffi_core::{query_iter, query_state, world};

use super::{SharedQueryIter, SharedQueryState};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_query_state_iter_mut(
    world_ptr: *mut world,
    query_ptr: *mut query_state,
    out_iter: *mut *mut query_iter,
) -> bool {
    if world_ptr.is_null() || query_ptr.is_null() {
        return false;
    }

    let world = unsafe { &mut *(world_ptr as *mut World) };
    let state = unsafe { &mut *(query_ptr as *mut SharedQueryState) };

    let iter: SharedQueryIter = state.iter_mut(world);

    unsafe {
        *out_iter = Box::into_raw(Box::new(iter)) as *mut query_iter;
    }

    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_query_state_drop(query_ptr: *mut query_state) {
    let _ = unsafe { Box::from_raw(query_ptr as *mut SharedQueryState) };
}
