use bevy::ecs::{
    query::{QueryIter, QueryState},
    world::FilteredEntityMut,
};

pub type SharedQueryState = QueryState<FilteredEntityMut<'static, 'static>>;

pub type SharedQueryIter = QueryIter<'static, 'static, FilteredEntityMut<'static, 'static>, ()>;
