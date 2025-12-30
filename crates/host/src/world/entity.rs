use bevy::ecs::world::FilteredEntityMut;

pub type SharedEntityRef = FilteredEntityMut<'static, 'static>;
