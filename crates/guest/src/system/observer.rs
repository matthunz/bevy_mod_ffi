use crate::system::{IntoSystem, System, SystemInput, SystemParam};
use bevy_ecs::{entity::Entity, event::Event};
use bevy_reflect::TypePath;
use bytemuck::Pod;
use std::ops::Deref;

pub trait SharedEvent: Event + Pod + TypePath {}

impl<E: Event + Pod + TypePath> SharedEvent for E {}

pub struct OnEntity<'a, E> {
    pub entity: Entity,
    pub event: &'a E,
}

impl<E> SystemInput for OnEntity<'_, E> {}

impl<E> Deref for OnEntity<'_, E> {
    type Target = E;

    fn deref(&self) -> &Self::Target {
        self.event
    }
}

pub struct On<'a, E> {
    pub event: &'a E,
}

impl<E> SystemInput for On<'_, E> {}

impl<E> Deref for On<'_, E> {
    type Target = E;

    fn deref(&self) -> &Self::Target {
        self.event
    }
}

pub trait ObserverSystem<E: SharedEvent>: System<In = On<'static, E>, Out = ()> {}

impl<E: SharedEvent, T: System<In = On<'static, E>, Out = ()>> ObserverSystem<E> for T {}

pub trait IntoObserverSystem<E: SharedEvent, Marker>:
    IntoSystem<Marker, In = On<'static, E>, Out = ()>
where
    Self::System: 'static,
    <Self::System as System>::Param: SystemParam + 'static,
{
}

impl<E, Marker, T> IntoObserverSystem<E, Marker> for T
where
    E: SharedEvent + 'static,
    T: IntoSystem<Marker, In = On<'static, E>, Out = ()>,
    T::System: 'static,
    <T::System as System>::Param: SystemParam + 'static,
{
}

pub trait EntityObserverSystem<E: SharedEvent>:
    System<In = OnEntity<'static, E>, Out = ()>
{
}

impl<E: SharedEvent, T: System<In = OnEntity<'static, E>, Out = ()>> EntityObserverSystem<E> for T {}

pub trait IntoEntityObserverSystem<E: SharedEvent, Marker>:
    IntoSystem<Marker, In = OnEntity<'static, E>, Out = ()>
where
    Self::System: 'static,
    <Self::System as System>::Param: SystemParam + 'static,
{
}

impl<E, Marker, T> IntoEntityObserverSystem<E, Marker> for T
where
    E: SharedEvent + 'static,
    T: IntoSystem<Marker, In = OnEntity<'static, E>, Out = ()>,
    T::System: 'static,
    <T::System as System>::Param: SystemParam + 'static,
{
}
