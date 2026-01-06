use bevy_mod_ffi_core::*;
use std::slice;
pub type ObserverClosure = Box<dyn FnMut(&[*mut dyn_system_param], *mut trigger)>;

unsafe extern "C" {
    pub fn bevy_system_state_build_on(
        world: *mut world,
        state_ptr: *mut system_state,
        event_name_ptr: *const u8,
        event_name_len: usize,
        f_ptr: *mut (),
        run_observer_fn: RunObserverFn,
    ) -> bool;
}

#[allow(clippy::missing_safety_doc)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_guest_run_observer(
    f_ptr: *mut (),
    params: *const *mut dyn_system_param,
    params_len: usize,
    trigger_ptr: *mut trigger,
) {
    let f = unsafe { &mut *(f_ptr as *mut ObserverClosure) };
    let params_slice = unsafe { slice::from_raw_parts(params, params_len) };
    f(params_slice, trigger_ptr);
}
