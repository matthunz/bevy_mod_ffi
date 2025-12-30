use std::slice;

mod builder;
pub use builder::{ParamBuilder, ParamCursor};

mod param;
pub use param::SystemParam;

mod state;
pub use state::{SystemRef, SystemState};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_guest_run_system(
    f_ptr: *mut (),
    params: *const (),
    params_len: usize,
) {
    let f = unsafe { &mut *(f_ptr as *mut Box<dyn FnMut(&[*const ()])>) };
    let params_slice = unsafe { slice::from_raw_parts(params as _, params_len) };
    f(params_slice);
}

pub trait System<Marker> {
    type In;
    type Out;
    type Param: SystemParam;

    fn run(
        &mut self,
        input: Self::In,
        params: <Self::Param as SystemParam>::Item<'_, '_>,
    ) -> Self::Out;
}

pub struct InputMarker;

macro_rules! impl_system_fn {
    ($($param:ident),*) => {
        #[allow(non_snake_case)]
        impl<Out, F, $($param: SystemParam,)*> System<fn($($param,)*) -> Out> for F
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
                call(self, $($param,)*)
            }
        }

        #[allow(non_snake_case)]
        impl<In, Out, Func, $($param: SystemParam,)*> System<(InputMarker, fn(In, $($param,)*) -> Out)> for Func
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
                call(self, input, $($param,)*)
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
