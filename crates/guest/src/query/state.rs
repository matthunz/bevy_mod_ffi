use super::{QueryBuilder, QueryData, QueryFilter, QueryIter};
use crate::world::World;
use bevy_mod_ffi_core::{query_iter, query_state};
use bevy_mod_ffi_guest_sys;
use std::{marker::PhantomData, ptr};

pub struct QueryState<D: QueryData, F: QueryFilter = ()> {
    pub(crate) ptr: *mut query_state,
    pub(crate) state: D::State,
    pub(crate) _marker: PhantomData<F>,
}

impl<D: QueryData, F: QueryFilter> QueryState<D, F> {
    pub fn new(world: &mut World) -> QueryState<D, F> {
        QueryBuilder::new(world).build()
    }

    pub fn from_raw(ptr: *mut query_state, data_state: D::State) -> Self {
        Self {
            ptr,
            state: data_state,
            _marker: PhantomData,
        }
    }

    pub fn iter_mut<'w, 's>(&'s mut self, world: &'w mut World) -> QueryIter<'w, 's, D, F> {
        let mut iter_ptr: *mut query_iter = ptr::null_mut();
        let success = unsafe {
            bevy_mod_ffi_guest_sys::query::state::bevy_query_state_iter_mut(
                world.ptr,
                self.ptr,
                &mut iter_ptr,
            )
        };

        if !success || iter_ptr.is_null() {
            panic!("Failed to create query iterator");
        }

        QueryIter::new(iter_ptr, &mut self.state)
    }
}

impl<D: QueryData, F: QueryFilter> Drop for QueryState<D, F> {
    fn drop(&mut self) {
        unsafe { bevy_mod_ffi_guest_sys::query::state::bevy_query_state_drop(self.ptr) }
    }
}
