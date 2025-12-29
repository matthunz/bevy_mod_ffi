use bevy::ecs::{
    prelude::*,
    system::{DynSystemParam, SystemState},
};

pub mod param;

type SharedSystemState = SystemState<Vec<DynSystemParam<'static, 'static>>>;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_system_state_get(
    world_ptr: *mut (),
    state_ptr: *mut (),
    out_params: *mut *mut [*mut ()],
    out_params_len: *mut i32,
) -> bool {
    let world = unsafe { &mut *(world_ptr as *mut World) };
    let system_state = unsafe { &mut *(state_ptr as *mut SharedSystemState) };

    let params: Vec<DynSystemParam> = system_state.get_mut(world);

    let mut pointers: Vec<*mut ()> = Vec::new();
    for param in params {
        let boxed = Box::new(param);
        pointers.push(Box::into_raw(boxed) as *mut ());
    }

    let len = pointers.len();
    let pointers = pointers.into_boxed_slice();

    unsafe {
        *out_params = Box::into_raw(pointers);
        *out_params_len = len as i32;
    }

    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_system_state_apply(world_ptr: *mut (), state_ptr: *mut ()) -> bool {
    let world = unsafe { &mut *(world_ptr as *mut World) };

    let system_state = unsafe { &mut *(state_ptr as *mut SharedSystemState) };
    system_state.apply(world);

    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_system_state_drop(state_ptr: *mut ()) {
    let _ = unsafe { Box::from_raw(state_ptr as *mut SharedSystemState) };
}
