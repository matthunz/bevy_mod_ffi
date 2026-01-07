use bevy_mod_ffi_core::*;

unsafe extern "C" {
    pub fn bevy_query_builder_new(world_ptr: *mut world) -> *mut query_builder;

    pub fn bevy_query_builder_with_ref(builder: *mut query_builder, component_id: usize);

    pub fn bevy_query_builder_with_mut(builder: *mut query_builder, component_id: usize);

    pub fn bevy_query_builder_with(builder: *mut query_builder, component_id: usize);

    pub fn bevy_query_builder_without(builder: *mut query_builder, component_id: usize);

    pub fn bevy_query_builder_build(builder: *mut query_builder) -> *mut query_state;

    pub fn bevy_query_builder_drop(builder: *mut query_builder);
}
