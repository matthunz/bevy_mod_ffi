use bevy_mod_ffi_core::*;

unsafe extern "C" {
    pub fn bevy_query_state_iter_mut(
        world: *mut world,
        query: *mut query_state,
        out_iter: *mut *mut query_iter,
    ) -> bool;

    pub fn bevy_query_state_drop(query: *mut query_state);
}
