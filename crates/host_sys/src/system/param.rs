use bevy::{
    ecs::{
        prelude::*,
        system::{DynParamBuilder, DynSystemParam, QueryParamBuilder},
        world::{FilteredEntityMut, World},
    },
    prelude::*,
};
use bevy_mod_ffi_core::{
    dyn_system_param, param_builder, query, query_builder, system_state, world,
};

type SharedQueryBuilder<'w> = QueryBuilder<'w, FilteredEntityMut<'static, 'static>>;

pub struct ParamBuilderAccumulator {
    pub builders: Vec<DynParamBuilder<'static>>,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_param_builder_new(
    _world_ptr: *mut world,
    out_builder: *mut *mut param_builder,
) -> bool {
    let accumulator = ParamBuilderAccumulator {
        builders: Vec::new(),
    };

    unsafe {
        *out_builder = Box::into_raw(Box::new(accumulator)) as *mut param_builder;
    }

    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_param_builder_add_query(
    builder_ptr: *mut param_builder,
    query_builder_ptr: *mut query_builder,
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
    world_ptr: *mut world,
    builder_ptr: *mut param_builder,
    out_state: *mut *mut system_state,
) -> bool {
    let world = unsafe { &mut *(world_ptr as *mut World) };
    let accumulator = unsafe { Box::from_raw(builder_ptr as *mut ParamBuilderAccumulator) };

    let system_state = accumulator.builders.build_state(world);
    unsafe {
        *out_state = Box::into_raw(Box::new(system_state)) as *mut system_state;
    }

    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_param_builder_drop(builder_ptr: *mut param_builder) {
    let _ = unsafe { Box::from_raw(builder_ptr as *mut ParamBuilderAccumulator) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_dyn_system_params_drop(param_ptr: *mut dyn_system_param) {
    let _ = unsafe { Box::from_raw(param_ptr) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_dyn_system_param_downcast_query(
    param_ptr: *mut dyn_system_param,
    out_query: *mut *mut query,
) -> bool {
    let param = unsafe { Box::from_raw(param_ptr as *mut DynSystemParam) };
    let query_param: Query<FilteredEntityMut> = param.downcast().unwrap();
    unsafe {
        *out_query = Box::into_raw(Box::new(query_param)) as *mut query;
    }
    true
}
