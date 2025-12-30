use super::{ParamBuilder, ParamCursor};
use crate::{
    Query, World,
    query::{QueryData, QueryFilter, QueryState},
};
use std::ptr;

pub unsafe trait SystemParam {
    type State: 'static;

    type Item<'w, 's>;

    fn build(world: &mut World, builder: &mut ParamBuilder) -> Self::State;

    unsafe fn get_param<'w, 's>(
        state: &'s mut Self::State,
        cursor: &mut ParamCursor<'_>,
    ) -> Self::Item<'w, 's>;
}

unsafe impl SystemParam for () {
    type State = ();
    type Item<'w, 's> = ();

    fn build(_world: &mut World, _builder: &mut ParamBuilder) {}

    unsafe fn get_param<'w, 's>(
        state: &'s mut Self::State,
        cursor: &mut ParamCursor<'_>,
    ) -> Self::Item<'w, 's> {
        let _ = state;
        let _ = cursor;
    }
}

unsafe impl<D, F> SystemParam for Query<'_, '_, D, F>
where
    D: QueryData + 'static,
    F: QueryFilter + 'static,
{
    type State = QueryState<D, F>;
    type Item<'w, 's> = Query<'w, 's, D, F>;

    fn build(world: &mut World, builder: &mut ParamBuilder) -> Self::State {
        builder.add_query::<D, F>(world);
        QueryState::new(world)
    }

    unsafe fn get_param<'w, 's>(
        state: &'s mut Self::State,
        cursor: &mut ParamCursor<'_>,
    ) -> Self::Item<'w, 's> {
        let ptr = cursor.next().unwrap_or(ptr::null_mut());
        Query::new(ptr, state)
    }
}

macro_rules! impl_system_param_tuple {
    ($($param:ident),*) => {
        #[allow(non_snake_case)]
        unsafe impl<$($param: SystemParam,)*> SystemParam for ($($param,)*) {
            type State = ($($param::State,)*);
            type Item<'w, 's> = ($($param::Item<'w, 's>,)*);

            fn build(world: &mut World, builder: &mut ParamBuilder) -> Self::State {
                ($($param::build(world, builder),)*)
            }

            unsafe fn get_param<'w, 's>(
                state: &'s mut Self::State,
                cursor: &mut ParamCursor<'_>,
            ) -> Self::Item<'w, 's> {
                let ($($param,)*) = state;
                unsafe { ($($param::get_param($param, cursor),)*) }
            }
        }
    };
}

impl_system_param_tuple!(P0);
impl_system_param_tuple!(P0, P1);
impl_system_param_tuple!(P0, P1, P2);
impl_system_param_tuple!(P0, P1, P2, P3);
impl_system_param_tuple!(P0, P1, P2, P3, P4);
impl_system_param_tuple!(P0, P1, P2, P3, P4, P5);
impl_system_param_tuple!(P0, P1, P2, P3, P4, P5, P6);
impl_system_param_tuple!(P0, P1, P2, P3, P4, P5, P6, P7);
