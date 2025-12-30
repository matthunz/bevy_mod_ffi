use bevy::{ecs::world::World, prelude::*};
use std::{any::TypeId, ffi::CStr, slice};

pub mod entity;

pub unsafe fn get_type_id(
    type_path_ptr: *const u8,
    type_path_len: usize,
    world: &World,
) -> Option<TypeId> {
    let type_path_bytes = unsafe { slice::from_raw_parts(type_path_ptr, type_path_len) };
    let type_path = CStr::from_bytes_with_nul(type_path_bytes)
        .unwrap()
        .to_str()
        .unwrap();

    let registry = world.get_resource::<AppTypeRegistry>().unwrap();
    let registry_ref = registry.read();
    let registration = match registry_ref.get_with_type_path(type_path) {
        Some(r) => r,
        None => {
            return None;
        }
    };

    let type_id = registration.type_id();
    Some(type_id)
}
