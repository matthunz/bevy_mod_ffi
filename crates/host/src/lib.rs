use bevy::ecs::component::ComponentId;
use bevy::ecs::world::World;
use bevy::prelude::*;
use std::ffi::CStr;
use std::slice;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_world_get_resource_id(
    world_ptr: *mut World,
    type_path_ptr: *const u8,
    type_path_len: usize,
    out_id: *mut usize,
) -> bool {
    let world = unsafe { &*(world_ptr as *const World) };

    let type_path_bytes = unsafe { slice::from_raw_parts(type_path_ptr, type_path_len as usize) };
    let type_path = CStr::from_bytes_with_nul(type_path_bytes)
        .unwrap()
        .to_str()
        .unwrap();

    let registry = world.get_resource::<AppTypeRegistry>().unwrap();
    let registry_ref = registry.read();
    let registration = match registry_ref.get_with_type_path(type_path) {
        Some(r) => r,
        None => {
            return false;
        }
    };
    let type_id = registration.type_id();
    drop(registry_ref);

    let Some(component_id) = world.components().get_resource_id(type_id) else {
        return false;
    };
    unsafe {
        *out_id = component_id.index();
    }

    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_world_get_resource(
    world_ptr: *mut World,
    component_id: usize,
    out_ptr: *mut *mut u8,
) -> bool {
    let world = unsafe { &*(world_ptr as *const World) };
    let id = ComponentId::new(component_id);

    let ptr = match world.get_resource_by_id(id) {
        Some(p) => p,
        None => {
            return false;
        }
    };

    unsafe {
        *out_ptr = ptr.as_ptr() as _;
    }

    true
}
