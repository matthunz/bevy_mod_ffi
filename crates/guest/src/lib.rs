pub mod query;

pub mod system;

pub mod world;

pub mod prelude {
    pub use bevy_ecs::{
        component::ComponentId,
        ptr::{Ptr, PtrMut},
    };

    pub use bytemuck::{Pod, Zeroable};

    pub use crate::query::{Query, QueryBuilder};

    pub use crate::system::{SystemParam, SystemState};

    pub use crate::world::World;
}
