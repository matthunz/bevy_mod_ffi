use crate::{DynamicHooks, SharedRegistry, SharedSystem, SystemIn};
use bevy::{
    ecs::{
        component::{ComponentCloneBehavior, ComponentDescriptor, ComponentId, StorageType},
        lifecycle::HookContext,
        world::{DeferredWorld, World},
    },
    prelude::*,
    ptr::OwningPtr,
};
use bevy_mod_ffi_core::{
    deferred_world, entity_world_mut, system, world, BundleComponent, ComponentHookFn,
};
use std::{
    alloc::{self, Layout},
    any::TypeId,
    ffi::CStr,
    ptr::{self, NonNull},
    slice,
};

fn run_guest_hook(
    hook_fn: ComponentHookFn,
    deferred: &mut DeferredWorld<'_>,
    context: &HookContext,
) {
    let deferred_ptr = deferred as *mut DeferredWorld as *mut deferred_world;
    let entity_bits = context.entity.to_bits();
    let component_id_idx = context.component_id.index();
    unsafe {
        hook_fn(deferred_ptr, entity_bits, component_id_idx);
    }
}

pub mod entity;

pub mod deferred;

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
    let world = unsafe { &mut *(world_ptr as *mut World) };

    let type_path_bytes = unsafe { slice::from_raw_parts(type_path_ptr, type_path_len) };
    let type_path = CStr::from_bytes_with_nul(type_path_bytes)
        .unwrap()
        .to_str()
        .unwrap();

    let component_id = if let Some(type_id) = get_type_id(type_path_ptr, type_path_len, world) {
        let Some(component_id) = world.components().get_id(type_id) else {
            return false;
        };
        component_id
    } else if let Some(registry) = world.get_resource::<SharedRegistry>() {
        let Some(component_id) = registry.get_component_id(type_path) else {
            return false;
        };
        component_id
    } else {
        return false;
    };

    unsafe {
        *out_id = component_id.index();
    }

    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_world_run_system(
    world_ptr: *mut world,
    system_ptr: *mut system,
    input_ptr: *mut u8,
    output_ptr: *mut u8,
) {
    let world = unsafe { &mut *(world_ptr as *mut World) };
    let system = unsafe { &mut *(system_ptr as *mut SharedSystem) };

    let input = SystemIn {
        input_ptr,
        output_ptr,
    };
    system.run(input, world).unwrap();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_world_register_component(
    world_ptr: *mut world,
    name_ptr: *const u8,
    name_len: usize,
    size: usize,
    align: usize,
    is_table: u8,
    on_add: Option<ComponentHookFn>,
    on_insert: Option<ComponentHookFn>,
    on_replace: Option<ComponentHookFn>,
    on_remove: Option<ComponentHookFn>,
    on_despawn: Option<ComponentHookFn>,
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

    let storage_type = if is_table != 0 {
        StorageType::Table
    } else {
        StorageType::SparseSet
    };

    let descriptor = unsafe {
        ComponentDescriptor::new_with_layout(
            name.clone(),
            storage_type,
            layout,
            None,
            true,
            ComponentCloneBehavior::Ignore,
        )
    };

    let id = world.register_component_with_descriptor(descriptor);

    let dynamic_hooks = DynamicHooks {
        on_add,
        on_insert,
        on_replace,
        on_remove,
        on_despawn,
    };

    {
        let mut registry = world.resource_mut::<SharedRegistry>();
        registry.type_path_to_id.insert(name, id);
        registry.hooks.insert(id, dynamic_hooks);
    }

    if let Some(hooks) = world.register_component_hooks_by_id(id) {
        if on_add.is_some() {
            hooks.on_add(|mut deferred: DeferredWorld<'_>, context: HookContext| {
                let hook_fn = deferred
                    .resource::<SharedRegistry>()
                    .hooks
                    .get(&context.component_id)
                    .and_then(|h| h.on_add);
                if let Some(hook_fn) = hook_fn {
                    run_guest_hook(hook_fn, &mut deferred, &context);
                }
            });
        }
        if on_insert.is_some() {
            hooks.on_insert(|mut deferred: DeferredWorld<'_>, context: HookContext| {
                let hook_fn = deferred
                    .resource::<SharedRegistry>()
                    .hooks
                    .get(&context.component_id)
                    .and_then(|h| h.on_insert);
                if let Some(hook_fn) = hook_fn {
                    run_guest_hook(hook_fn, &mut deferred, &context);
                }
            });
        }
        if on_replace.is_some() {
            hooks.on_replace(|mut deferred: DeferredWorld<'_>, context: HookContext| {
                let hook_fn = deferred
                    .resource::<SharedRegistry>()
                    .hooks
                    .get(&context.component_id)
                    .and_then(|h| h.on_replace);
                if let Some(hook_fn) = hook_fn {
                    run_guest_hook(hook_fn, &mut deferred, &context);
                }
            });
        }
        if on_remove.is_some() {
            hooks.on_remove(|mut deferred: DeferredWorld<'_>, context: HookContext| {
                let hook_fn = deferred
                    .resource::<SharedRegistry>()
                    .hooks
                    .get(&context.component_id)
                    .and_then(|h| h.on_remove);
                if let Some(hook_fn) = hook_fn {
                    run_guest_hook(hook_fn, &mut deferred, &context);
                }
            });
        }
        if on_despawn.is_some() {
            hooks.on_despawn(|mut deferred: DeferredWorld<'_>, context: HookContext| {
                let hook_fn = deferred
                    .resource::<SharedRegistry>()
                    .hooks
                    .get(&context.component_id)
                    .and_then(|h| h.on_despawn);
                if let Some(hook_fn) = hook_fn {
                    run_guest_hook(hook_fn, &mut deferred, &context);
                }
            });
        }
    }

    unsafe {
        *out_id = id.index();
    }

    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_world_spawn(
    world_ptr: *mut world,
    components_ptr: *const BundleComponent,
    component_len: usize,
    out_entity: *mut u64,
    out_entity_world_mut_ptr: *mut *mut entity_world_mut,
) -> bool {
    let world = unsafe { &mut *(world_ptr as *mut World) };
    let components = unsafe { slice::from_raw_parts(components_ptr, component_len) };

    let mut components_data = Vec::new();
    for component in components {
        let component_id = ComponentId::new(component.component_id);
        let component_info = world.components().get_info(component_id).unwrap();
        let layout = component_info.layout();

        let buffer = alloc::alloc(layout);
        unsafe {
            ptr::copy(component.ptr, buffer, layout.size());
        }

        components_data.push((component_id, buffer));
    }

    let mut entity = world.spawn_empty();
    for (component_id, buffer) in components_data {
        unsafe {
            let owning_ptr = OwningPtr::new(NonNull::new_unchecked(buffer));
            entity.insert_by_id(component_id, owning_ptr);
        }
    }

    unsafe {
        *out_entity = entity.id().to_bits();
        *out_entity_world_mut_ptr = Box::into_raw(Box::new(entity)) as *mut entity_world_mut;
    }

    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_world_entity_mut(
    world_ptr: *mut world,
    entity_bits: u64,
    out_entity_world_mut_ptr: *mut *mut entity_world_mut,
) -> bool {
    let world = unsafe { &mut *(world_ptr as *mut World) };
    let entity = Entity::from_bits(entity_bits);

    let entity_mut = match world.get_entity_mut(entity) {
        Ok(e) => e,
        Err(_) => return false,
    };

    unsafe {
        *out_entity_world_mut_ptr = Box::into_raw(Box::new(entity_mut)) as *mut entity_world_mut;
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

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_world_trigger_event(
    world_ptr: *mut world,
    event_name_ptr: *const u8,
    event_name_len: usize,
    event_data_ptr: *const u8,
    event_data_len: usize,
) -> bool {
    let world = unsafe { &mut *(world_ptr as *mut World) };

    let event_name_bytes = unsafe { slice::from_raw_parts(event_name_ptr, event_name_len) };
    let event_name = match CStr::from_bytes_with_nul(event_name_bytes) {
        Ok(cstr) => match cstr.to_str() {
            Ok(s) => s,
            Err(_) => return false,
        },
        Err(_) => return false,
    };

    let event_data = unsafe { slice::from_raw_parts(event_data_ptr, event_data_len) };

    let mut registry = world.remove_resource::<SharedRegistry>().unwrap();

    if let Some(event_ops) = registry.events.remove(event_name) {
        event_ops.trigger(world, event_data);
        let key = event_ops.type_path();
        registry.events.insert(key, event_ops);
        world.insert_resource(registry);
        true
    } else {
        world.insert_resource(registry);
        false
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_world_trigger_event_targets(
    world_ptr: *mut world,
    event_name_ptr: *const u8,
    event_name_len: usize,
    event_data_ptr: *const u8,
    event_data_len: usize,
    entity_bits: u64,
) -> bool {
    let world = unsafe { &mut *(world_ptr as *mut World) };
    let entity = Entity::from_bits(entity_bits);

    let event_name_bytes = unsafe { slice::from_raw_parts(event_name_ptr, event_name_len) };
    let event_name = match CStr::from_bytes_with_nul(event_name_bytes) {
        Ok(cstr) => match cstr.to_str() {
            Ok(s) => s,
            Err(_) => return false,
        },
        Err(_) => return false,
    };

    let event_data = unsafe { slice::from_raw_parts(event_data_ptr, event_data_len) };

    let mut registry = world.remove_resource::<SharedRegistry>().unwrap();
    if let Some(event_ops) = registry.events.remove(event_name) {
        event_ops.trigger_for_entity(world, event_data, entity);
        let key = event_ops.type_path();
        registry.events.insert(key, event_ops);
        world.insert_resource(registry);
        true
    } else {
        world.insert_resource(registry);
        false
    }
}
