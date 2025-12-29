use bevy::{
    ecs::{
        component::ComponentId,
        world::{FilteredEntityMut, World},
    },
    prelude::*,
};

type SharedQueryBuilder<'w> = QueryBuilder<'w, FilteredEntityMut<'static, 'static>>;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_query_builder_new(
    world_ptr: *mut (),
    out_builder: *mut *mut (),
) -> bool {
    let world = unsafe { &mut *(world_ptr as *mut World) };
    let builder = SharedQueryBuilder::new(world);

    unsafe {
        *out_builder = Box::into_raw(Box::new(builder)) as *mut ();
    }

    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_query_builder_with_ref(
    builder_ptr: *mut (),
    component_id: usize,
) -> bool {
    let builder = unsafe { &mut *(builder_ptr as *mut SharedQueryBuilder) };
    builder.ref_id(ComponentId::new(component_id));

    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_query_builder_with_mut(
    builder_ptr: *mut (),
    component_id: usize,
) -> bool {
    let builder = unsafe { &mut *(builder_ptr as *mut SharedQueryBuilder) };
    builder.mut_id(ComponentId::new(component_id));

    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_query_builder_with(
    builder_ptr: *mut (),
    component_id: usize,
) -> bool {
    let builder = unsafe { &mut *(builder_ptr as *mut SharedQueryBuilder) };
    builder.with_id(ComponentId::new(component_id));

    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_query_builder_without(
    builder_ptr: *mut (),
    component_id: usize,
) -> bool {
    let builder = unsafe { &mut *(builder_ptr as *mut SharedQueryBuilder) };
    builder.without_id(ComponentId::new(component_id));

    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_query_builder_build(
    builder_ptr: *mut (),
    out_state: *mut *mut (),
) -> bool {
    let mut builder = unsafe { Box::from_raw(builder_ptr as *mut SharedQueryBuilder) };

    let query_state = builder.build();

    unsafe {
        *out_state = Box::into_raw(Box::new(query_state)) as *mut ();
    }

    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_query_builder_drop(builder_ptr: *mut ()) {
    let _ = unsafe { Box::from_raw(builder_ptr as *mut SharedQueryBuilder) };
}
