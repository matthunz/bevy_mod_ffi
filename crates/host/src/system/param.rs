use bevy::ecs::{
    prelude::*,
    system::{DynParamBuilder, QueryParamBuilder},
    world::FilteredEntityMut,
};

type SharedQueryBuilder<'w> = QueryBuilder<'w, FilteredEntityMut<'static, 'static>>;

pub struct ParamBuilderAccumulator {
    builders: Vec<DynParamBuilder<'static>>,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_param_builder_new(
    _world_ptr: *mut (),
    out_builder: *mut *mut (),
) -> bool {
    let accumulator = ParamBuilderAccumulator {
        builders: Vec::new(),
    };

    unsafe {
        *out_builder = Box::into_raw(Box::new(accumulator)) as *mut ();
    }

    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_param_builder_add_query(
    builder_ptr: *mut (),
    query_builder_ptr: *mut (),
) -> bool {
    let accumulator = unsafe { &mut *(builder_ptr as *mut ParamBuilderAccumulator) };
    let query_builder = unsafe { Box::from_raw(query_builder_ptr as *mut SharedQueryBuilder) };

    let dyn_builder = DynParamBuilder::new(QueryParamBuilder::new(
        move |params: &mut SharedQueryBuilder| {
            params.extend_access(query_builder.access().clone());
        },
    ));

    accumulator.builders.push(dyn_builder);

    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_param_builder_build(
    world_ptr: *mut (),
    builder_ptr: *mut (),
    out_state: *mut *mut (),
) -> bool {
    let world = unsafe { &mut *(world_ptr as *mut World) };
    let accumulator = unsafe { Box::from_raw(builder_ptr as *mut ParamBuilderAccumulator) };

    let system_state = accumulator.builders.build_state(world);
    unsafe {
        *out_state = Box::into_raw(Box::new(system_state)) as *mut ();
    }

    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_param_builder_drop(builder_ptr: *mut ()) {
    let _ = unsafe { Box::from_raw(builder_ptr as *mut ParamBuilderAccumulator) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_dyn_system_params_drop(param_ptr: *mut ()) {
    let _ = unsafe { Box::from_raw(param_ptr) };
}
