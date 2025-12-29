use bevy_ecs::prelude::*;
use bevy_reflect::Reflect;
use bytemuck::{Pod, Zeroable};

#[derive(Clone, Copy, Pod, Zeroable, Reflect, Resource)]
#[repr(C)]
pub struct ExampleResource {
    pub value: i32,
}
