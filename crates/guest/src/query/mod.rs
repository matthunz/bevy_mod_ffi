use crate::world::{FilteredEntityMut, World};
use bevy_ecs::entity::Entity;
use bevy_mod_ffi_core::{filtered_entity_mut, query, query_iter};
use bevy_mod_ffi_guest_sys;
use std::{marker::PhantomData, ptr};

mod builder;
pub use builder::QueryBuilder;

mod data;
pub use data::QueryData;

mod filter;
pub use filter::{QueryFilter, With, Without};

mod iter;
pub use iter::QueryIter;

mod state;
pub use state::QueryState;

pub struct Query<'w, 's, D: QueryData, F: QueryFilter = ()> {
    ptr: *mut query,
    state: &'s mut D::State,
    _marker: PhantomData<(&'w mut World, &'s mut QueryState<D, F>)>,
}

impl<'w, 's, D: QueryData, F: QueryFilter> Query<'w, 's, D, F> {
    pub(crate) fn new(ptr: *mut query, state: &'s mut D::State) -> Self {
        Self {
            ptr,
            state,
            _marker: PhantomData,
        }
    }

    pub fn iter_mut<'a>(&'a mut self) -> QueryIter<'a, 'a, D, F> {
        let mut iter_ptr: *mut query_iter = ptr::null_mut();

        let success =
            unsafe { bevy_mod_ffi_guest_sys::query::bevy_query_iter_mut(self.ptr, &mut iter_ptr) };
        if !success || iter_ptr.is_null() {
            panic!("Failed to create query iterator");
        }

        QueryIter::new(iter_ptr, self.state)
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<D::Item<'_, '_>> {
        let mut ptr: *mut filtered_entity_mut = ptr::null_mut();

        let success = unsafe {
            bevy_mod_ffi_guest_sys::query::bevy_query_get_mut(self.ptr, entity.to_bits(), &mut ptr)
        };
        if !success {
            return None;
        }

        let mut entity_mut = unsafe { FilteredEntityMut::from_ptr(entity, ptr) };
        Some(D::from_entity(&mut entity_mut, self.state))
    }

    pub fn get_entity_mut(&mut self, entity: Entity) -> Option<FilteredEntityMut<'_>> {
        let mut ptr: *mut filtered_entity_mut = ptr::null_mut();

        let success = unsafe {
            bevy_mod_ffi_guest_sys::query::bevy_query_get_mut(self.ptr, entity.to_bits(), &mut ptr)
        };
        if !success {
            return None;
        }

        Some(unsafe { FilteredEntityMut::from_ptr(entity, ptr) })
    }
}

impl<D: QueryData, F: QueryFilter> Drop for Query<'_, '_, D, F> {
    fn drop(&mut self) {
        unsafe { bevy_mod_ffi_guest_sys::query::bevy_query_drop(self.ptr) }
    }
}
