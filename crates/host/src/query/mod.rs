use bevy::ecs::{system::Query, world::FilteredEntityMut};

pub mod builder;

pub mod iter;

pub mod state;

#[unsafe(no_mangle)]
unsafe extern "C" fn bevy_query_iter_mut(query_ptr: *mut (), out_iter: *mut *mut ()) -> bool {
    let query = unsafe { &mut *(query_ptr as *mut Query<FilteredEntityMut>) };
    let iter = query.iter_mut();

    unsafe {
        *out_iter = Box::into_raw(Box::new(iter)) as *mut ();
    }

    true
}

#[unsafe(no_mangle)]
unsafe extern "C" fn bevy_query_drop(query_ptr: *mut ()) {
    let _ = unsafe { Box::from_raw(query_ptr as *mut Query<FilteredEntityMut>) };
}
