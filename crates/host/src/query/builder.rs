use bevy::{ecs::world::FilteredEntityMut, prelude::*};

pub type SharedQueryBuilder<'w> = QueryBuilder<'w, FilteredEntityMut<'static, 'static>>;
