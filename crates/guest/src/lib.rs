pub mod component;

pub mod query;

pub mod system;

pub mod world;

pub mod prelude {
    pub use bevy_ecs::{
        component::ComponentId,
        entity::Entity,
        ptr::{Ptr, PtrMut},
    };

    pub use bytemuck::{Pod, Zeroable};

    pub use crate::component::SharedComponent;

    pub use crate::query::{Query, QueryBuilder, With, Without};

    pub use crate::system::{
        IntoObserverSystem, IntoSystem, ObserverSystem, On, SharedEvent, System, SystemParam,
        SystemRef, SystemState,
    };

    pub use crate::world::World;
}
