pub mod component;

pub mod query;

pub mod system;

pub mod world;

pub mod prelude {
    pub use bevy_ecs::{
        component::ComponentId,
        entity::{Entity, EntityMapper},
        ptr::{Ptr, PtrMut},
    };

    pub use bevy_reflect::{Reflect, TypePath};

    pub use bytemuck::{Pod, Zeroable};

    pub use crate::component::{
        ComponentCloneBehavior, ComponentMutability, HookContext, Immutable, Mutable,
        RequiredComponentsRegistrator, SharedComponent, StorageType,
    };

    pub use crate::query::{Query, QueryBuilder, With, Without};

    pub use crate::system::{
        EntityObserverSystem, IntoEntityObserverSystem, IntoObserverSystem, IntoSystem,
        ObserverSystem, On, OnEntity, SharedEvent, System, SystemParam, SystemRef, SystemState,
    };

    pub use crate::world::{DeferredWorld, World};

    #[cfg(feature = "macros")]
    pub use bevy_mod_ffi_macros::SharedComponent;
}
