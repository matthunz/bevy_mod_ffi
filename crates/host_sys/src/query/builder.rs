use bevy::{
    ecs::{component::ComponentId, world::World},
    prelude::*,
};
use bevy_mod_ffi_core::{query_builder, query_state, world};

use super::SharedQueryBuilder;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_query_builder_new(world_ptr: *mut world) -> *mut query_builder {
    let world = unsafe { &mut *(world_ptr as *mut World) };
    let builder = SharedQueryBuilder::new(world);
    Box::into_raw(Box::new(builder)) as *mut query_builder
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_query_builder_with_ref(
    builder_ptr: *mut query_builder,
    component_id: usize,
) {
    let builder = unsafe { &mut *(builder_ptr as *mut SharedQueryBuilder) };
    builder.ref_id(ComponentId::new(component_id));
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_query_builder_with_mut(
    builder_ptr: *mut query_builder,
    component_id: usize,
) {
    let builder = unsafe { &mut *(builder_ptr as *mut SharedQueryBuilder) };
    builder.mut_id(ComponentId::new(component_id));
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_query_builder_with(
    builder_ptr: *mut query_builder,
    component_id: usize,
) {
    let builder = unsafe { &mut *(builder_ptr as *mut SharedQueryBuilder) };
    builder.with_id(ComponentId::new(component_id));
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_query_builder_without(
    builder_ptr: *mut query_builder,
    component_id: usize,
) {
    let builder = unsafe { &mut *(builder_ptr as *mut SharedQueryBuilder) };
    builder.without_id(ComponentId::new(component_id));
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_query_builder_build(
    builder_ptr: *mut query_builder,
) -> *mut query_state {
    let mut builder = unsafe { Box::from_raw(builder_ptr as *mut SharedQueryBuilder) };
    let query_state = builder.build();

    Box::into_raw(Box::new(query_state)) as *mut query_state
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_query_builder_drop(builder_ptr: *mut query_builder) {
    let _ = unsafe { Box::from_raw(builder_ptr as *mut SharedQueryBuilder) };
}
