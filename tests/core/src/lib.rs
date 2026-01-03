use bevy_ecs::prelude::*;
use bevy_mod_ffi_guest::prelude::*;
use bevy_reflect::Reflect;
use bytemuck::{Pod, Zeroable};

#[derive(Component, Clone, Copy, Debug, Pod, Zeroable, Reflect)]
#[repr(C)]
pub struct Counter {
    pub value: i32,
}

impl SharedComponent for Counter {}

#[repr(C)]
#[derive(Component, Clone, Copy, Debug, Zeroable, Pod, Reflect)]
pub struct TestMarker;

impl SharedComponent for TestMarker {}
