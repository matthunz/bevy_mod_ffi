use crate::world::DeferredWorld;
use bevy_ecs::component::ComponentId;
use bevy_ecs::entity::{Entity, EntityMapper};
use bevy_reflect::TypePath;
use bytemuck::Pod;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageType {
    Table,
    SparseSet,
}

pub trait ComponentMutability: 'static {}

pub struct Mutable;
impl ComponentMutability for Mutable {}

pub struct Immutable;
impl ComponentMutability for Immutable {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HookContext {
    pub entity: Entity,
    pub component_id: ComponentId,
    pub caller: Option<&'static std::panic::Location<'static>>,
}

#[derive(Clone)]
pub enum ComponentCloneBehavior {
    Default,
    Ignore,
    Custom(fn(&SourceComponent, &mut ComponentCloneCtx)),
}

pub struct SourceComponent;
pub struct ComponentCloneCtx;

pub struct RequiredComponentsRegistrator<'a, 'b> {
    _marker: std::marker::PhantomData<(&'a (), &'b ())>,
}

pub trait SharedComponent: Pod + TypePath + Sized + Send + Sync + 'static {
    type Mutability: ComponentMutability;

    const STORAGE_TYPE: StorageType;

    fn on_add() -> Option<for<'w> fn(DeferredWorld<'w>, HookContext)> {
        None
    }

    fn on_insert() -> Option<for<'w> fn(DeferredWorld<'w>, HookContext)> {
        None
    }

    fn on_replace() -> Option<for<'w> fn(DeferredWorld<'w>, HookContext)> {
        None
    }

    fn on_remove() -> Option<for<'w> fn(DeferredWorld<'w>, HookContext)> {
        None
    }

    fn on_despawn() -> Option<for<'w> fn(DeferredWorld<'w>, HookContext)> {
        None
    }

    fn register_required_components(
        _component_id: ComponentId,
        _required_components: &mut RequiredComponentsRegistrator<'_, '_>,
    ) {
    }

    fn clone_behavior() -> ComponentCloneBehavior {
        ComponentCloneBehavior::Default
    }

    fn map_entities<E>(_this: &mut Self, _mapper: &mut E)
    where
        E: EntityMapper,
    {
    }
}
