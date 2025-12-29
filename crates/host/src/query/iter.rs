use super::state::SharedQueryIter;
use bevy::prelude::*;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_query_iter_next(
    iter_ptr: *mut (),
    out_entity_id: *mut u64,
    out_entity: *mut *mut (),
) -> bool {
    let shared_iter = unsafe { &mut *(iter_ptr as *mut SharedQueryIter) };

    let entity_mut = match shared_iter.next() {
        Some(e) => e,
        None => return false,
    };
    unsafe {
        *out_entity_id = entity_mut.id().to_bits();
        *out_entity = Box::into_raw(Box::new(entity_mut)) as *mut ();
    }

    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_query_iter_drop(iter_ptr: *mut ()) {
    let _ = unsafe { Box::from_raw(iter_ptr as *mut SharedQueryIter) };
}
