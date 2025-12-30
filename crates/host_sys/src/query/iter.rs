use bevy::prelude::*;
use bevy_mod_ffi_core::{filtered_entity_mut, query_iter};

use super::SharedQueryIter;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_query_iter_next(
    iter_ptr: *mut query_iter,
    out_entity_id: *mut u64,
    out_entity: *mut *mut filtered_entity_mut,
) -> bool {
    let shared_iter = unsafe { &mut *(iter_ptr as *mut SharedQueryIter) };

    let entity_mut = match shared_iter.next() {
        Some(e) => e,
        None => return false,
    };
    unsafe {
        *out_entity_id = entity_mut.id().to_bits();
        *out_entity = Box::into_raw(Box::new(entity_mut)) as *mut filtered_entity_mut;
    }

    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_query_iter_drop(iter_ptr: *mut query_iter) {
    let _ = unsafe { Box::from_raw(iter_ptr as *mut SharedQueryIter) };
}
