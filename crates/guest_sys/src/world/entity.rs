use bevy_mod_ffi_core::*;

unsafe extern "C" {
    pub fn bevy_filtered_entity_mut_get_component(
        entity: *mut filtered_entity_mut,
        component_id: usize,
        out_ptr: *mut *mut u8,
    ) -> bool;

    pub fn bevy_filtered_entity_mut_get_component_mut(
        entity: *mut filtered_entity_mut,
        component_id: usize,
        out_ptr: *mut *mut u8,
    ) -> bool;

    pub fn bevy_filtered_entity_mut_drop(entity: *mut filtered_entity_mut);
}
