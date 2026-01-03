use bevy_ecs::prelude::*;
use bevy_mod_ffi_guest::prelude::*;
use bevy_reflect::Reflect;
use bytemuck::{Pod, Zeroable};

#[derive(Component, Clone, Copy, Debug, Pod, Zeroable, Reflect)]
#[repr(C)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl SharedComponent for Position {
    type Mutability = Mutable;
    const STORAGE_TYPE: StorageType = StorageType::Table;
}

#[derive(Component, Clone, Copy, Debug, Pod, Zeroable, Reflect)]
#[repr(C)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl SharedComponent for Velocity {
    type Mutability = Mutable;
    const STORAGE_TYPE: StorageType = StorageType::Table;
}

#[derive(Event, Clone, Copy, Debug, Pod, Zeroable, Reflect)]
#[repr(C)]
pub struct Damage {
    pub amount: f32,
}
