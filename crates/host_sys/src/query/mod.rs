use bevy::{
    ecs::{
        prelude::*,
        query::{QueryIter, QueryState},
        world::FilteredEntityMut,
    },
    prelude::*,
};
use bevy_mod_ffi_core::{query, query_iter};

type SharedQueryBuilder<'w> = QueryBuilder<'w, FilteredEntityMut<'static, 'static>>;
type SharedQueryState = QueryState<FilteredEntityMut<'static, 'static>>;
type SharedQueryIter = QueryIter<'static, 'static, FilteredEntityMut<'static, 'static>, ()>;

pub mod builder;
pub mod iter;
pub mod state;

#[unsafe(no_mangle)]
unsafe extern "C" fn bevy_query_iter_mut(
    query_ptr: *mut query,
    out_iter: *mut *mut query_iter,
) -> bool {
    let query = unsafe { &mut *(query_ptr as *mut Query<FilteredEntityMut>) };
    let iter = query.iter_mut();

    unsafe {
        *out_iter = Box::into_raw(Box::new(iter)) as *mut query_iter;
    }

    true
}

#[unsafe(no_mangle)]
unsafe extern "C" fn bevy_query_drop(query_ptr: *mut query) {
    let _ = unsafe { Box::from_raw(query_ptr as *mut Query<FilteredEntityMut>) };
}
