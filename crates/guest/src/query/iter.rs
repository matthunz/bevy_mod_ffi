use super::{QueryData, QueryFilter, QueryState};
use crate::world::{FilteredEntityMut, World};
use bevy_ecs::entity::Entity;
use std::marker::PhantomData;

unsafe extern "C" {
    fn bevy_query_iter_next(
        iter: *mut (),
        out_entity_id: *mut u64,
        out_entity: *mut *mut (),
    ) -> bool;

    fn bevy_query_iter_drop(iter: *mut ());
}

pub struct QueryIter<'w, 's, D: QueryData, F: QueryFilter> {
    iter_ptr: *mut (),
    state: &'s mut D::State,
    _marker: PhantomData<(&'w mut World, &'s QueryState<D, F>)>,
}

impl<'w, 's, D: QueryData, F: QueryFilter> QueryIter<'w, 's, D, F> {
    pub(crate) fn new(iter_ptr: *mut (), state: &'s mut D::State) -> Self {
        QueryIter {
            iter_ptr,
            state,
            _marker: PhantomData,
        }
    }
}

impl<'w, 's, D: QueryData, F: QueryFilter> Iterator for QueryIter<'w, 's, D, F> {
    type Item = D::Item<'w, 's>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut entity_id: u64 = 0;
        let mut entity_ptr: *mut () = std::ptr::null_mut();

        let success =
            unsafe { bevy_query_iter_next(self.iter_ptr, &mut entity_id, &mut entity_ptr) };

        if !success {
            return None;
        }

        let state: &'s mut D::State =
            unsafe { &mut *(self.state as *const D::State as *mut D::State) };

        let mut entity =
            unsafe { FilteredEntityMut::from_ptr(Entity::from_bits(entity_id), entity_ptr) };
        let item = D::from_entity(&mut entity, state);

        Some(item)
    }
}

impl<D: QueryData, F: QueryFilter> Drop for QueryIter<'_, '_, D, F> {
    fn drop(&mut self) {
        if !self.iter_ptr.is_null() {
            unsafe { bevy_query_iter_drop(self.iter_ptr) };
        }
    }
}
