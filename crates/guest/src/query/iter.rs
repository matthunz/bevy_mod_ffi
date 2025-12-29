use super::{QueryData, QueryFilter, QueryState};
use crate::{FilteredEntityMut, World};
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
    world: &'w mut World,
    _marker: PhantomData<&'s QueryState<D, F>>,
}

impl<'w, 's, D: QueryData, F: QueryFilter> QueryIter<'w, 's, D, F> {
    pub(crate) fn new(iter_ptr: *mut (), state: &'s mut D::State, world: &'w mut World) -> Self {
        QueryIter {
            iter_ptr,
            state,
            world,
            _marker: PhantomData,
        }
    }
}

impl<'w, 's, D: QueryData, F: QueryFilter> Iterator for QueryIter<'w, 's, D, F> {
    type Item = (Entity, D::Item<'w, 's>);

    fn next(&mut self) -> Option<Self::Item> {
        let mut entity_id: u64 = 0;
        let mut entity_ptr: *mut () = std::ptr::null_mut();

        let success =
            unsafe { bevy_query_iter_next(self.iter_ptr, &mut entity_id, &mut entity_ptr) };

        if !success {
            return None;
        }

        let world: &'w mut World = unsafe { &mut *(self.world as *const World as *mut World) };
        let state: &'s mut D::State =
            unsafe { &mut *(self.state as *const D::State as *mut D::State) };

        let mut entity = unsafe { FilteredEntityMut::from_ptr(entity_ptr, world) };
        let item = D::from_entity(&mut entity, state);

        Some((Entity::from_bits(entity_id), item))
    }
}

impl<D: QueryData, F: QueryFilter> Drop for QueryIter<'_, '_, D, F> {
    fn drop(&mut self) {
        if !self.iter_ptr.is_null() {
            unsafe { bevy_query_iter_drop(self.iter_ptr) };
        }
    }
}
