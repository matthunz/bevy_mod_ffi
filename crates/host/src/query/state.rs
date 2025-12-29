use bevy::{
    ecs::{
        query::{QueryIter, QueryState},
        world::{FilteredEntityMut, World},
    },
    prelude::*,
};

pub type SharedQueryState = QueryState<FilteredEntityMut<'static, 'static>>;

pub(crate) type SharedQueryIter =
    QueryIter<'static, 'static, FilteredEntityMut<'static, 'static>, ()>;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_query_state_iter_mut(
    world_ptr: *mut (),
    query_ptr: *mut (),
    out_iter: *mut *mut (),
) -> bool {
    if world_ptr.is_null() || query_ptr.is_null() {
        return false;
    }

    let world = unsafe { &mut *(world_ptr as *mut World) };
    let state = unsafe { &mut *(query_ptr as *mut SharedQueryState) };

    let iter: SharedQueryIter = state.iter_mut(world);

    unsafe {
        *out_iter = Box::into_raw(Box::new(iter)) as *mut ();
    }

    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_query_state_drop(query_ptr: *mut ()) {
    let _ = unsafe { Box::from_raw(query_ptr as *mut SharedQueryState) };
}
