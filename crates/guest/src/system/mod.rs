use bevy_mod_ffi_core::dyn_system_param;
use std::slice;

mod builder;
pub use builder::{ParamBuilder, ParamCursor};

mod param;
pub use param::SystemParam;

mod state;
pub use state::{SystemRef, SystemState};

#[allow(clippy::missing_safety_doc)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_guest_run_system(
    f_ptr: *mut (),
    params: *const *mut dyn_system_param,
    params_len: usize,
) {
    let f = unsafe { &mut *(f_ptr as *mut Box<dyn FnMut(&[*mut dyn_system_param])>) };
    let params_slice = unsafe { slice::from_raw_parts(params, params_len) };
    f(params_slice);
}

pub trait System {
    type In;
    type Out;
    type Param: SystemParam;

    fn run(
        &mut self,
        input: Self::In,
        params: <Self::Param as SystemParam>::Item<'_, '_>,
    ) -> Self::Out;
}

pub trait IntoSystem<Marker> {
    type In;
    type Out;
    type System: System<In = Self::In, Out = Self::Out>;

    fn into_system(self) -> Self::System;
}

pub struct InputMarker;

pub struct FunctionSystem<F, Marker> {
    f: F,
    _marker: std::marker::PhantomData<fn() -> Marker>,
}

macro_rules! impl_system_fn {
    ($($param:ident),*) => {
        #[allow(non_snake_case, clippy::too_many_arguments)]
        impl<Out, F, $($param: SystemParam,)*> System for FunctionSystem<F, fn($($param,)*) -> Out>
        where
            F: Send + Sync + 'static,
            for<'a> &'a mut F:
                FnMut($($param,)*) -> Out +
                FnMut($($param::Item<'_, '_>,)*) -> Out,
            Out: 'static,
        {
            type In = ();
            type Out = Out;
            type Param = ($($param,)*);

            fn run(
                &mut self,
                _input: Self::In,
                param_value: <Self::Param as SystemParam>::Item<'_, '_>,
            ) -> Self::Out {
                fn call<Out, $($param,)*>(
                    mut f: impl FnMut($($param,)*) -> Out,
                    $($param: $param,)*
                ) -> Out {
                    f($($param,)*)
                }
                let ($($param,)*) = param_value;
                call(&mut self.f, $($param,)*)
            }
        }

        #[allow(non_snake_case, clippy::too_many_arguments)]
        impl<Out, F, $($param: SystemParam,)*> IntoSystem<fn($($param,)*) -> Out> for F
        where
            F: Send + Sync + 'static,
            for<'a> &'a mut F:
                FnMut($($param,)*) -> Out +
                FnMut($($param::Item<'_, '_>,)*) -> Out,
            Out: 'static,
        {
            type In = ();
            type Out = Out;
            type System = FunctionSystem<F, fn($($param,)*) -> Out>;

            fn into_system(self) -> Self::System {
                FunctionSystem {
                    f: self,
                    _marker: std::marker::PhantomData,
                }
            }
        }

        #[allow(non_snake_case, clippy::too_many_arguments)]
        impl<In, Out, Func, $($param: SystemParam,)*> System for FunctionSystem<Func, (InputMarker, fn(In, $($param,)*) -> Out)>
        where
            Func: Send + Sync + 'static,
            for<'a> &'a mut Func:
                FnMut(In, $($param,)*) -> Out +
                FnMut(In, $($param::Item<'_, '_>,)*) -> Out,
            In: 'static,
            Out: 'static,
        {
            type In = In;
            type Out = Out;
            type Param = ($($param,)*);

            fn run(
                &mut self,
                input: Self::In,
                param_value: <Self::Param as SystemParam>::Item<'_, '_>,
            ) -> Self::Out {
                fn call<In, Out, $($param,)*>(
                    mut f: impl FnMut(In, $($param,)*) -> Out,
                    input: In,
                    $($param: $param,)*
                ) -> Out {
                    f(input, $($param,)*)
                }
                let ($($param,)*) = param_value;
                call(&mut self.f, input, $($param,)*)
            }
        }

        #[allow(non_snake_case, clippy::too_many_arguments)]
        impl<In, Out, Func, $($param: SystemParam,)*> IntoSystem<(InputMarker, fn(In, $($param,)*) -> Out)> for Func
        where
            Func: Send + Sync + 'static,
            for<'a> &'a mut Func:
                FnMut(In, $($param,)*) -> Out +
                FnMut(In, $($param::Item<'_, '_>,)*) -> Out,
            In: 'static,
            Out: 'static,
        {
            type In = In;
            type Out = Out;
            type System = FunctionSystem<Func, (InputMarker, fn(In, $($param,)*) -> Out)>;

            fn into_system(self) -> Self::System {
                FunctionSystem {
                    f: self,
                    _marker: std::marker::PhantomData,
                }
            }
        }
    };
}

impl_system_fn!(P0);
impl_system_fn!(P0, P1);
impl_system_fn!(P0, P1, P2);
impl_system_fn!(P0, P1, P2, P3);
impl_system_fn!(P0, P1, P2, P3, P4);
impl_system_fn!(P0, P1, P2, P3, P4, P5);
impl_system_fn!(P0, P1, P2, P3, P4, P5, P6);
impl_system_fn!(P0, P1, P2, P3, P4, P5, P6, P7);
