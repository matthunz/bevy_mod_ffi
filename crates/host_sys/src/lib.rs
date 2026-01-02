use bevy::{
    ecs::{component::ComponentId, resource::Resource},
    platform::collections::HashMap,
};

pub mod query;
pub use query::*;

pub mod system;
pub use system::*;

pub mod world;
pub use world::*;

#[derive(Resource, Default)]
pub struct DynamicComponentRegistry {
    type_path_to_id: HashMap<String, ComponentId>,
}
