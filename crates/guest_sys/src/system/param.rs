use bevy_mod_ffi_core::*;

unsafe extern "C" {
    pub fn bevy_param_builder_new(
        world_ptr: *mut world,
        out_builder: *mut *mut param_builder,
    ) -> bool;

    pub fn bevy_param_builder_add_query(
        builder: *mut param_builder,
        query_ptr: *mut query_builder,
    ) -> bool;

    pub fn bevy_param_builder_build(
        world_ptr: *mut world,
        builder: *mut param_builder,
        out_state: *mut *mut system_state,
    ) -> bool;

    pub fn bevy_param_builder_drop(builder: *mut param_builder);

    pub fn bevy_dyn_system_param_downcast_query(
        param_ptr: *mut dyn_system_param,
        out_query: *mut *mut query,
    ) -> bool;
}
