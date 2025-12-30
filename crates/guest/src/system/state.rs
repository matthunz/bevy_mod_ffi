use super::{ParamBuilder, SystemParam, bevy_guest_run_system};
use crate::{
    system::{ParamCursor, System},
    world::World,
};
use std::{marker::PhantomData, ptr, slice};

unsafe extern "C" {
    pub fn bevy_system_state_get(
        world: *mut (),
        state: *mut (),
        out_params: *mut *mut *mut (),
        out_params_len: *mut i32,
    ) -> bool;

    pub fn bevy_system_state_apply(world: *mut (), state: *mut ()) -> bool;

    pub fn bevy_system_state_build(
        state: *mut (),
        f_ptr: *mut (),
        run_system_fn: unsafe extern "C" fn(*mut (), *const (), usize),
        out_ptr: *mut (),
    );

    pub fn bevy_system_state_drop(state: *mut ());

    pub fn bevy_dyn_system_params_drop(param: *mut ());
}

pub struct SystemState<P: SystemParam> {
    pub(crate) ptr: *mut (),
    pub(crate) state: P::State,
    _marker: PhantomData<fn() -> P>,
}

impl<P: SystemParam> SystemState<P> {
    pub fn new(world: &mut World) -> Self {
        let mut builder = ParamBuilder::new(world);
        let state = P::build(world, &mut builder);
        let state_ptr = builder.build();

        Self {
            ptr: state_ptr,
            state,
            _marker: PhantomData,
        }
    }

    pub fn get<'w, 's>(&'s mut self, world: &'w mut World) -> P::Item<'w, 's> {
        let mut params_ptr: *mut *mut () = ptr::null_mut();
        let mut params_len: i32 = 0;

        let success =
            unsafe { bevy_system_state_get(world.ptr, self.ptr, &mut params_ptr, &mut params_len) };

        if !success {
            panic!("Failed to get system state");
        }

        let params = if params_ptr.is_null() || params_len == 0 {
            &[]
        } else {
            unsafe { slice::from_raw_parts(params_ptr, params_len as usize) }
        };

        let mut cursor = ParamCursor::new(params);
        let out = unsafe { P::get_param(&mut self.state, &mut cursor) };

        unsafe { bevy_dyn_system_params_drop(params_ptr as *mut ()) };
        out
    }

    pub fn apply(&mut self, world: &mut World) {
        if !self.ptr.is_null() {
            unsafe {
                bevy_system_state_apply(world.ptr, self.ptr);
            }
        }
    }

    pub fn state(&self) -> &P::State {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut P::State {
        &mut self.state
    }

    pub fn build<Marker, Out, S>(mut self, mut system: S) -> SystemRef<S>
    where
        S: System<Marker, In = (), Out = Out, Param = P>,
    {
        let state_ptr = self.ptr;
        self.ptr = ptr::null_mut();

        #[allow(clippy::type_complexity)]
        let system_boxed: Box<dyn FnMut(&[*mut ()])> = Box::new(move |params| {
            let mut param_cursor = ParamCursor::new(params);
            let params =
                unsafe { <S::Param as SystemParam>::get_param(&mut self.state, &mut param_cursor) };
            system.run((), params);
        });

        let mut out_ptr: *mut () = ptr::null_mut();
        unsafe {
            bevy_system_state_build(
                state_ptr,
                Box::into_raw(Box::new(system_boxed)) as _,
                bevy_guest_run_system,
                &mut out_ptr as *mut _ as *mut (),
            )
        }

        SystemRef {
            ptr: out_ptr,
            _marker: PhantomData,
        }
    }
}

impl<P: SystemParam> Drop for SystemState<P> {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { bevy_system_state_drop(self.ptr) };
        }
    }
}

pub struct SystemRef<F> {
    pub(crate) ptr: *mut (),
    _marker: PhantomData<F>,
}

impl<F> SystemRef<F> {}
