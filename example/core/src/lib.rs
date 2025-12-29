use bevy_ecs::prelude::*;
use bevy_reflect::Reflect;
use bytemuck::{Pod, Zeroable};

#[derive(Resource, Clone, Copy, Debug, Pod, Zeroable, Reflect)]
#[repr(C)]
pub struct ExampleResource {
    pub value: i32,
}

#[derive(Component, Clone, Copy, Debug, Pod, Zeroable, Reflect)]
#[repr(C)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Clone, Copy, Debug, Pod, Zeroable, Reflect)]
#[repr(C)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}
