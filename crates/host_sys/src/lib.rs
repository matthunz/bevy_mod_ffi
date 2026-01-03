use bevy::{
    ecs::{component::ComponentId, entity::Entity, event::Event, resource::Resource},
    platform::collections::HashMap,
    reflect::TypePath,
};

pub mod query;
pub use query::*;

pub mod system;
use system::observer::{Observable, ObservableOf};
pub use system::*;

pub mod world;
pub use world::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LibraryId(pub u64);

#[derive(Resource, Default)]
pub struct SharedRegistry {
    type_path_to_id: HashMap<String, ComponentId>,
    events: HashMap<&'static str, Box<dyn Observable>>,
    library_observers: HashMap<LibraryId, Vec<Entity>>,
    current_library_id: Option<LibraryId>,
    next_library_id: u64,
}

impl SharedRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_library_id(&mut self) -> LibraryId {
        let id = LibraryId(self.next_library_id);
        self.next_library_id += 1;
        self.library_observers.insert(id, Vec::new());
        id
    }

    pub fn set_current_library(&mut self, id: Option<LibraryId>) {
        self.current_library_id = id;
    }

    pub fn current_library_id(&self) -> Option<LibraryId> {
        self.current_library_id
    }

    pub fn register_observer(&mut self, observer: Entity) {
        if let Some(lib_id) = self.current_library_id {
            if let Some(observers) = self.library_observers.get_mut(&lib_id) {
                observers.push(observer);
            }
        }
    }

    pub fn take_library_observers(&mut self, lib_id: LibraryId) -> Option<Vec<Entity>> {
        self.library_observers.remove(&lib_id)
    }

    pub fn register_event<E: Event + TypePath + Clone + Copy>(&mut self)
    where
        for<'a> E::Trigger<'a>: Default,
    {
        self.events
            .insert(E::type_path(), Box::new(ObservableOf::<E>::new()));
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
