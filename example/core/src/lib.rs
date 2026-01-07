use bevy_ecs::prelude::*;
use bevy_mod_ffi::prelude::*;
use bevy_reflect::Reflect;
use bytemuck::{Pod, Zeroable};

#[derive(Component, SharedComponent, Clone, Copy, Debug, Pod, Zeroable, Reflect)]
#[repr(C)]
pub struct Health {
    pub current: f32,
}

#[derive(Event, Clone, Copy, Debug, Pod, Zeroable, Reflect)]
#[repr(C)]
pub struct Damage {
    pub amount: f32,
}
