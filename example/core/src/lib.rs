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

/// A dynamic component that stores an array of u64 values.
/// This demonstrates how to create components with custom sizes.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct DynamicComponent {
    pub data: [u64; 8],
}

impl DynamicComponent {
    pub fn new(data: [u64; 8]) -> Self {
        Self { data }
    }

    pub fn zeroed() -> Self {
        Self { data: [0; 8] }
    }

    pub fn from_slice(values: &[u64]) -> Self {
        let mut data = [0u64; 8];
        let len = values.len().min(8);
        data[..len].copy_from_slice(&values[..len]);
        Self { data }
    }
}
