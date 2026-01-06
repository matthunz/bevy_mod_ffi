use bevy::{
    ecs::{
        prelude::*,
        system::{DynParamBuilder, DynSystemParam, ParamBuilder, QueryParamBuilder},
        world::{DeferredWorld, FilteredEntityMut, World},
    },
    prelude::*,
};
use bevy_mod_ffi_core::{
    commands, deferred_world, dyn_system_param, param_builder, query, query_builder, system_state,
    world, RunCommandFn,
};

use crate::SharedSystemState;

type SharedQueryBuilder<'w> = QueryBuilder<'w, FilteredEntityMut<'static, 'static>>;

pub struct ParamBuilderAccumulator {
    pub builders: Vec<DynParamBuilder<'static>>,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_param_builder_new(out_builder: *mut *mut param_builder) -> bool {
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

    // Clone the access before the query_builder is dropped, to avoid holding a reference to the world
    let access = query_builder.access().clone();
    drop(query_builder);

    let dyn_builder = DynParamBuilder::new(QueryParamBuilder::new(
        move |params: &mut SharedQueryBuilder| {
            params.extend_access(access.clone());
        },
    ));

    accumulator.builders.push(dyn_builder);

    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_param_builder_add_commands(builder_ptr: *mut param_builder) -> bool {
    let accumulator = unsafe { &mut *(builder_ptr as *mut ParamBuilderAccumulator) };

    let dyn_builder = DynParamBuilder::new::<Commands>(ParamBuilder);

    accumulator.builders.push(dyn_builder);

    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_param_builder_add_deferred_world(
    builder_ptr: *mut param_builder,
) -> bool {
    let accumulator = unsafe { &mut *(builder_ptr as *mut ParamBuilderAccumulator) };

    let dyn_builder = DynParamBuilder::new::<DeferredWorld>(ParamBuilder);

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

    let system_state: SharedSystemState = (accumulator.builders,).build_state(world);
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
    let _ = unsafe { Box::from_raw(param_ptr as *mut DynSystemParam) };
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

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_dyn_system_param_downcast_commands(
    param_ptr: *mut dyn_system_param,
    out_commands: *mut *mut commands,
) -> bool {
    let param = unsafe { Box::from_raw(param_ptr as *mut DynSystemParam) };
    let commands_param: Commands = param.downcast().unwrap();
    unsafe {
        *out_commands = Box::into_raw(Box::new(commands_param)) as *mut commands;
    }
    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_dyn_system_param_downcast_deferred_world(
    param_ptr: *mut dyn_system_param,
    out_deferred: *mut *mut deferred_world,
) -> bool {
    let param = unsafe { Box::from_raw(param_ptr as *mut DynSystemParam) };
    let deferred_param: DeferredWorld = param.downcast().unwrap();
    unsafe {
        *out_deferred = Box::into_raw(Box::new(deferred_param)) as *mut deferred_world;
    }
    true
}

struct SharedCommand {
    f_ptr: usize,
    run_command_fn: RunCommandFn,
}

unsafe impl Send for SharedCommand {}

impl Command for SharedCommand {
    fn apply(self, world: &mut World) {
        unsafe { (self.run_command_fn)(self.f_ptr as *mut (), world as *mut World as *mut world) };
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_commands_push(
    commands_ptr: *mut commands,
    _world_ptr: *mut world,
    f_ptr: *mut (),
    run_command_fn: RunCommandFn,
) -> bool {
    let commands = unsafe { &mut *(commands_ptr as *mut Commands) };

    let command = SharedCommand {
        f_ptr: f_ptr as usize,
        run_command_fn,
    };

    commands.queue(command);

    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_commands_drop(commands_ptr: *mut commands) {
    let _ = unsafe { Box::from_raw(commands_ptr as *mut Commands) };
}
