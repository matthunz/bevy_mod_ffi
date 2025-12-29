use super::{ParamBuilder, SystemParam};
use crate::{World, system::ParamCursor};
use std::marker::PhantomData;

unsafe extern "C" {
    pub fn bevy_system_state_get(
        world: *mut (),
        state: *mut (),
        out_params: *mut *mut *mut (),
        out_params_len: *mut i32,
    ) -> bool;

    pub fn bevy_system_state_apply(world: *mut (), state: *mut ()) -> bool;

    pub fn bevy_system_state_drop(state: *mut ());

    pub fn bevy_dyn_system_params_drop(param: *mut ());
}

pub type SystemParamItem<'w, 's, P> = <P as SystemParam>::Item<'w, 's>;

pub struct SystemState<P: SystemParam + 'static> {
    pub(crate) state_ptr: *mut (),
    pub(crate) state: P::State,
    _marker: PhantomData<fn() -> P>,
}

impl<P: SystemParam + 'static> SystemState<P> {
    /// Create a new SystemState by building params on the host side
    pub fn new(world: &mut World) -> Self {
        let mut builder = ParamBuilder::new(world);
        P::build_builder(world, &mut builder);
        let state_ptr = builder.build();
        let state = P::build_state(world);

        Self {
            state_ptr,
            state,
            _marker: PhantomData,
        }
    }

    pub fn from_raw(state_ptr: *mut (), state: P::State) -> Self {
        Self {
            state_ptr,
            state,
            _marker: PhantomData,
        }
    }

    pub fn get<'w, 's>(&'s mut self, world: &'w mut World) -> SystemParamItem<'w, 's, P> {
        let mut params_ptr: *mut *mut () = std::ptr::null_mut();
        let mut params_len: i32 = 0;

        let success = unsafe {
            bevy_system_state_get(world.ptr, self.state_ptr, &mut params_ptr, &mut params_len)
        };

        if !success {
            panic!("Failed to get system state");
        }

        let params = if params_ptr.is_null() || params_len == 0 {
            &[]
        } else {
            unsafe { std::slice::from_raw_parts(params_ptr, params_len as usize) }
        };

        let mut cursor = ParamCursor::new(params);
        let out = unsafe { P::get_param(&mut self.state, &mut cursor, world) };

        unsafe { bevy_dyn_system_params_drop(params_ptr as *mut ()) };
        out
    }

    pub fn apply(&mut self, world: &mut World) {
        if !self.state_ptr.is_null() {
            unsafe {
                bevy_system_state_apply(world.ptr, self.state_ptr);
            }
        }
    }

    pub fn state(&self) -> &P::State {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut P::State {
        &mut self.state
    }
}

impl<P: SystemParam + 'static> Drop for SystemState<P> {
    fn drop(&mut self) {
        if !self.state_ptr.is_null() {
            unsafe { bevy_system_state_drop(self.state_ptr) };
        }
    }
}
