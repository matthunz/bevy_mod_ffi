use bevy_mod_ffi_core::*;

unsafe extern "C" {
    pub fn bevy_system_state_get(
        world: *mut world,
        state: *mut system_state,
        out_params: *mut *mut *mut dyn_system_param,
        out_params_len: *mut i32,
    ) -> bool;

    pub fn bevy_system_state_apply(world: *mut world, state: *mut system_state) -> bool;

    pub fn bevy_system_state_build(
        state: *mut system_state,
        f_ptr: *mut (),
        run_system_fn: RunSystemFn,
        out_ptr: *mut *mut system,
    );

    pub fn bevy_system_state_drop(state: *mut system_state);

    pub fn bevy_dyn_system_params_drop(param: *mut dyn_system_param);
}
