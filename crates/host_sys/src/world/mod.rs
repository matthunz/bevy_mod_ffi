use bevy::{
    ecs::{
        component::{ComponentCloneBehavior, ComponentDescriptor, ComponentId, StorageType},
        world::World,
    },
    prelude::*,
};
use bevy_mod_ffi_core::{system, world};
use std::{alloc::Layout, any::TypeId, ffi::CStr, slice};

pub mod entity;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_world_get_resource_id(
    world_ptr: *mut world,
    type_path_ptr: *const u8,
    type_path_len: usize,
    out_id: *mut usize,
) -> bool {
    let world = unsafe { &*(world_ptr as *const World) };

    let Some(type_id) = get_type_id(type_path_ptr, type_path_len, world) else {
        return false;
    };
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
    world_ptr: *mut world,
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

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_world_get_component_id(
    world_ptr: *mut world,
    type_path_ptr: *const u8,
    type_path_len: usize,
    out_id: *mut usize,
) -> bool {
    let world = unsafe { &*(world_ptr as *const World) };

    let Some(type_id) = get_type_id(type_path_ptr, type_path_len, world) else {
        return false;
    };
    let Some(component_id) = world.components().get_id(type_id) else {
        return false;
    };

    unsafe {
        *out_id = component_id.index();
    }

    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_world_run_system(world_ptr: *mut world, system_ptr: *mut system) {
    let world = unsafe { &mut *(world_ptr as *mut World) };
    let system = unsafe { &mut *(system_ptr as *mut Box<dyn System<In = (), Out = ()>>) };

    system.run((), world).unwrap();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_world_register_component(
    world_ptr: *mut world,
    name_ptr: *const u8,
    name_len: usize,
    size: usize,
    align: usize,
    is_table: bool,
    out_id: *mut usize,
) -> bool {
    let world = unsafe { &mut *(world_ptr as *mut World) };

    let name_bytes = unsafe { slice::from_raw_parts(name_ptr, name_len) };
    let name = CStr::from_bytes_with_nul(name_bytes)
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let layout = match Layout::from_size_align(size, align) {
        Ok(l) => l,
        Err(_) => return false,
    };

    let storage_type = if is_table {
        StorageType::Table
    } else {
        StorageType::SparseSet
    };
    let descriptor = unsafe {
        ComponentDescriptor::new_with_layout(
            name,
            storage_type,
            layout,
            None,
            true,
            ComponentCloneBehavior::Default,
        )
    };

    let id = world.register_component_with_descriptor(descriptor);
    unsafe {
        *out_id = id.index();
    }

    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_world_spawn_empty(
    world_ptr: *mut world,
    out_entity: *mut u64,
) -> bool {
    let world = unsafe { &mut *(world_ptr as *mut World) };

    let entity = world.spawn_empty().id();

    unsafe {
        *out_entity = entity.to_bits();
    }

    true
}

fn get_type_id(type_path_ptr: *const u8, type_path_len: usize, world: &World) -> Option<TypeId> {
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
