use crate::{
    query::QueryBuilder,
    world::{FilteredEntityMut, World},
};
use bevy_ecs::component::ComponentId;
use bevy_reflect::TypePath;
use bytemuck::Pod;

pub trait QueryData: Sized {
    type Item<'w, 's>;
    type State: Clone;

    fn build_query(builder: &mut QueryBuilder);

    fn build_state(world: &mut World) -> Self::State;

    fn from_entity<'w, 's>(
        entity: &mut FilteredEntityMut<'w>,
        state: &'s mut Self::State,
    ) -> Self::Item<'w, 's>;
}

impl QueryData for () {
    type Item<'w, 's> = ();
    type State = ();

    fn build_query(builder: &mut QueryBuilder) {
        let _ = builder;
    }

    fn build_state(world: &mut World) -> Self::State {
        let _ = world;
    }

    fn from_entity<'w, 's>(
        entity: &mut FilteredEntityMut<'w>,
        state: &'s mut Self::State,
    ) -> Self::Item<'w, 's> {
        let _ = entity;
        let _ = state;
    }
}

impl<T: TypePath + Pod + 'static> QueryData for &T {
    type Item<'w, 's> = &'w T;
    type State = ComponentId;

    fn build_query(builder: &mut QueryBuilder) {
        builder.with_ref::<T>();
    }

    fn build_state(world: &mut World) -> Self::State {
        world
            .get_component_id_from_type_path(T::type_path())
            .unwrap()
    }

    fn from_entity<'w, 's>(
        entity: &mut FilteredEntityMut<'w>,
        state: &'s mut Self::State,
    ) -> Self::Item<'w, 's> {
        let ptr = entity.get_by_id(*state).unwrap();
        unsafe { ptr.deref() }
    }
}

impl<T: TypePath + Pod + 'static> QueryData for &mut T {
    type Item<'w, 's> = &'w mut T;
    type State = ComponentId;

    fn build_query(builder: &mut QueryBuilder) {
        builder.with_mut::<T>();
    }

    fn build_state(world: &mut World) -> Self::State {
        world
            .get_component_id_from_type_path(T::type_path())
            .unwrap()
    }

    fn from_entity<'w, 's>(
        entity: &mut FilteredEntityMut<'w>,
        state: &'s mut Self::State,
    ) -> Self::Item<'w, 's> {
        let ptr = entity.get_mut_by_id(*state).unwrap();
        unsafe { ptr.deref_mut() }
    }
}

macro_rules! impl_query_data_tuple {
    ($($items:ident),+) => {
        impl<$($items: QueryData),+> QueryData for ($($items),+) {
            type Item<'w, 's> = ($($items::Item<'w, 's>),+);
            type State = ($($items::State),+);

            fn build_query(builder: &mut QueryBuilder) {
                $(
                    $items::build_query(builder);
                )+
            }

            fn build_state(world: &mut World) -> Self::State {
                ($($items::build_state(world)),+)
            }

            fn from_entity<'w, 's>(entity: &mut FilteredEntityMut<'w>, state: &'s mut Self::State) -> Self::Item<'w, 's> {
                #[allow(non_snake_case)]
                let ($($items),+) = state;
                (
                    $(
                        $items::from_entity(entity, $items)
                    ),+
                )
            }
        }
    };
}

impl_query_data_tuple!(A, B);
impl_query_data_tuple!(A, B, C);
impl_query_data_tuple!(A, B, C, D);
impl_query_data_tuple!(A, B, C, D, E);
impl_query_data_tuple!(A, B, C, D, E, F);
impl_query_data_tuple!(A, B, C, D, E, F, G);
impl_query_data_tuple!(A, B, C, D, E, F, G, H);
