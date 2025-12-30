use bevy_mod_ffi_core::*;

unsafe extern "C" {
    pub fn bevy_query_iter_next(
        iter: *mut query_iter,
        out_entity_id: *mut u64,
        out_entity: *mut *mut filtered_entity_mut,
    ) -> bool;

    pub fn bevy_query_iter_drop(iter: *mut query_iter);
}
