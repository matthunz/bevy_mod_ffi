use bevy::{
    ecs::{component::ComponentId, event::Event, resource::Resource},
    platform::collections::HashMap,
    reflect::TypePath,
};

pub mod query;
pub use query::*;

pub mod system;
use system::observer::{Observable, TypedEventOps};
pub use system::*;

pub mod world;
pub use world::*;

#[derive(Resource, Default)]
pub struct SharedRegistry {
    pub(crate) type_path_to_id: HashMap<String, ComponentId>,
    pub(crate) events: HashMap<&'static str, Box<dyn Observable>>,
}

impl SharedRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_event<E: Event + TypePath + Clone + Copy>(&mut self)
    where
        for<'a> E::Trigger<'a>: Default,
    {
        self.events
            .insert(E::type_path(), Box::new(TypedEventOps::<E>::new()));
    }

    pub fn is_event_registered(&self, name: &str) -> bool {
        self.events.contains_key(name)
    }

    pub fn get_event(&self, name: &str) -> Option<&dyn Observable> {
        self.events.get(name).map(|e| e.as_ref())
    }

    pub fn get_component_id(&self, type_path: &str) -> Option<ComponentId> {
        self.type_path_to_id.get(type_path).copied()
    }
}
