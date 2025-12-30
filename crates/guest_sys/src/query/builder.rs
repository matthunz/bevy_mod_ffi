use bevy_mod_ffi_core::*;

unsafe extern "C" {
    pub fn bevy_query_builder_new(
        world_ptr: *mut world,
        out_builder: *mut *mut query_builder,
    ) -> bool;

    pub fn bevy_query_builder_with_ref(builder: *mut query_builder, component_id: usize) -> bool;

    pub fn bevy_query_builder_with_mut(builder: *mut query_builder, component_id: usize) -> bool;

    pub fn bevy_query_builder_with(builder: *mut query_builder, component_id: usize) -> bool;

    pub fn bevy_query_builder_without(builder: *mut query_builder, component_id: usize) -> bool;

    pub fn bevy_query_builder_build(
        builder: *mut query_builder,
        out_state: *mut *mut query_state,
    ) -> bool;

    pub fn bevy_query_builder_drop(builder: *mut query_builder);
}
