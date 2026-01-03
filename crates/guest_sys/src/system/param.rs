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

    pub fn bevy_param_builder_add_commands(builder: *mut param_builder) -> bool;

    pub fn bevy_param_builder_add_deferred_world(builder: *mut param_builder) -> bool;

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

    pub fn bevy_dyn_system_param_downcast_commands(
        param_ptr: *mut dyn_system_param,
        out_commands: *mut *mut commands,
    ) -> bool;

    pub fn bevy_dyn_system_param_downcast_deferred_world(
        param_ptr: *mut dyn_system_param,
        out_deferred: *mut *mut deferred_world,
    ) -> bool;

    pub fn bevy_commands_push(
        commands_ptr: *mut commands,
        world_ptr: *mut world,
        f_ptr: *mut (),
        run_command_fn: RunCommandFn,
    ) -> bool;

    pub fn bevy_commands_drop(commands_ptr: *mut commands);
}

#[allow(clippy::missing_safety_doc)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_guest_run_command(f_ptr: *mut (), world_ptr: *mut world) {
    type CommandClosure = Box<dyn FnOnce(*mut world)>;

    let f = unsafe { Box::from_raw(f_ptr as *mut CommandClosure) };
    (*f)(world_ptr);
}
