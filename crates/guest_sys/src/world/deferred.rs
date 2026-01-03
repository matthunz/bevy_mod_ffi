use bevy_mod_ffi_core::*;

unsafe extern "C" {
    pub fn bevy_deferred_world_query(
        deferred_ptr: *mut deferred_world,
        query_state_ptr: *mut query_state,
        out_query: *mut *mut query,
    ) -> bool;

    pub fn bevy_deferred_world_get_mut(
        deferred_ptr: *mut deferred_world,
        entity_bits: u64,
        component_id: usize,
        out_ptr: *mut *mut u8,
    ) -> bool;

    pub fn bevy_deferred_world_get_resource_mut(
        deferred_ptr: *mut deferred_world,
        component_id: usize,
        out_ptr: *mut *mut u8,
    ) -> bool;

    pub fn bevy_deferred_world_drop(deferred_ptr: *mut deferred_world);
}
