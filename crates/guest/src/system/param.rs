use std::ptr;

use super::{ParamBuilder, ParamCursor};
use crate::{
    Query, World,
    query::{QueryData, QueryFilter, QueryState},
};

pub unsafe trait SystemParam {
    type State: 'static;

    type Item<'w, 's>;

    /// Build the host-side param builder by adding this param's requirements
    fn build_builder(world: &mut World, builder: &mut ParamBuilder);

    fn build_state(world: &mut World) -> Self::State;

    unsafe fn get_param<'w, 's>(
        state: &'s mut Self::State,
        cursor: &mut ParamCursor<'_>,
        world: &'w mut World,
    ) -> Self::Item<'w, 's>;
}

unsafe impl SystemParam for () {
    type State = ();
    type Item<'w, 's> = ();

    fn build_builder(_world: &mut World, _builder: &mut ParamBuilder) {
        // No params to add for unit type
    }

    fn build_state(_world: &mut World) -> Self::State {}

    unsafe fn get_param<'w, 's>(
        state: &'s mut Self::State,
        cursor: &mut ParamCursor<'_>,
        _world: &'w mut World,
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

    fn build_builder(world: &mut World, builder: &mut ParamBuilder) {
        builder.add_query::<D, F>(world);
    }

    fn build_state(world: &mut World) -> Self::State {
        QueryState::new(world)
    }

    unsafe fn get_param<'w, 's>(
        state: &'s mut Self::State,
        cursor: &mut ParamCursor<'_>,
        world: &'w mut World,
    ) -> Self::Item<'w, 's> {
        let ptr = cursor.next().unwrap_or(ptr::null_mut());
        Query::new(ptr, state, world)
    }
}
