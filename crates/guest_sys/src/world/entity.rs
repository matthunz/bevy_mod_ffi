use bevy_mod_ffi_core::*;

unsafe extern "C" {
    pub fn bevy_world_entity_mut_drop(entity: *mut entity_world_mut);

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

    pub fn bevy_entity_world_mut_observe(
        entity: *mut entity_world_mut,
        state_ptr: *mut system_state,
        event_name_ptr: *const u8,
        event_name_len: usize,
        f_ptr: *mut (),
        run_observer_fn: RunObserverFn,
    ) -> bool;

    pub fn bevy_entity_world_mut_trigger(
        entity: *mut entity_world_mut,
        event_name_ptr: *const u8,
        event_name_len: usize,
        event_data_ptr: *const u8,
        event_data_len: usize,
    ) -> bool;
}
