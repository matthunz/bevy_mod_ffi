use bevy::{
    ecs::{
        prelude::*,
        system::{DynSystemParam, SystemState},
        world::World,
    },
    prelude::*,
};
use bevy_mod_ffi_core::{dyn_system_param, system, system_state, world, RunSystemFn};

pub mod observer;

pub mod param;

pub type SharedSystem = Box<dyn System<In = In<SystemIn>, Out = ()>>;

pub type SharedSystemState = SystemState<(Vec<DynSystemParam<'static, 'static>>,)>;

pub struct SystemIn {
    pub input_ptr: *mut u8,
    pub output_ptr: *mut u8,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_system_state_get(
    world_ptr: *mut world,
    state_ptr: *mut system_state,
    out_params: *mut *mut *mut dyn_system_param,
    out_params_len: *mut i32,
) -> bool {
    let bevy_world = unsafe { &mut *(world_ptr as *mut World) };
    let system_state = unsafe { &mut *(state_ptr as *mut SharedSystemState) };

    let params: Vec<DynSystemParam> = system_state.get_mut(bevy_world).0;

    let mut pointers: Vec<*mut dyn_system_param> = Vec::new();
    for param in params {
        let boxed = Box::new(param);
        pointers.push(Box::into_raw(boxed) as *mut dyn_system_param);
    }

    let len = pointers.len();
    let pointers = pointers.into_boxed_slice();

    unsafe {
        *out_params = Box::into_raw(pointers) as *mut *mut dyn_system_param;
        *out_params_len = len as i32;
    }

    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_system_state_apply(
    world_ptr: *mut world,
    state_ptr: *mut system_state,
) -> bool {
    let world = unsafe { &mut *(world_ptr as *mut World) };

    let system_state = unsafe { &mut *(state_ptr as *mut SharedSystemState) };
    system_state.apply(world);

    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_system_state_drop(state_ptr: *mut system_state) {
    let _ = unsafe { Box::from_raw(state_ptr as *mut SharedSystemState) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_system_state_build(
    state_ptr: *mut system_state,
    f_ptr: *mut (),
    run_system_fn: RunSystemFn,
    out_ptr: *mut *mut system,
) {
    let state: Box<SharedSystemState> = unsafe { Box::from_raw(state_ptr as _) };

    let f_ptr_n = f_ptr as usize;

    let bevy_system =
        state.build_system_with_input(move |input: In<SystemIn>, params: Vec<DynSystemParam>| {
            let mut param_ptrs: Vec<*mut dyn_system_param> = Vec::new();
            for param in params {
                let boxed = Box::new(param);
                param_ptrs.push(Box::into_raw(boxed) as *mut dyn_system_param);
            }
            let len = param_ptrs.len();
            let pointers_ptr = param_ptrs.as_ptr();

            unsafe {
                run_system_fn(
                    f_ptr_n as _,
                    pointers_ptr,
                    len,
                    input.input_ptr,
                    input.output_ptr,
                )
            };
        });

    let boxed: Box<SharedSystem> = Box::new(Box::new(bevy_system));
    unsafe {
        *out_ptr = Box::into_raw(boxed) as *mut system;
    }
}
