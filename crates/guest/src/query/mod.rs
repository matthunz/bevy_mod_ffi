use crate::World;
use std::ptr;

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

unsafe extern "C" {
    pub fn bevy_query_iter_mut(query: *mut (), out_iter: *mut *mut ()) -> bool;

    pub fn bevy_query_drop(iter: *mut ());
}

pub struct Query<'w, 's, D: QueryData, F: QueryFilter = ()> {
    ptr: *mut (),
    state: &'s mut QueryState<D, F>,
    world: &'w mut World,
}

impl<'w, 's, D: QueryData, F: QueryFilter> Query<'w, 's, D, F> {
    pub(crate) fn new(ptr: *mut (), state: &'s mut QueryState<D, F>, world: &'w mut World) -> Self {
        Self { state, ptr, world }
    }

    pub fn iter_mut<'a>(&'a mut self) -> QueryIter<'a, 'a, D, F> {
        let mut iter_ptr: *mut () = ptr::null_mut();

        let success = unsafe { bevy_query_iter_mut(self.ptr, &mut iter_ptr) };
        if !success || iter_ptr.is_null() {
            panic!("Failed to create query iterator");
        }

        QueryIter::new(iter_ptr, &mut self.state.state, self.world)
    }
}

impl<D: QueryData, F: QueryFilter> Drop for Query<'_, '_, D, F> {
    fn drop(&mut self) {
        unsafe { bevy_query_drop(self.ptr) }
    }
}
