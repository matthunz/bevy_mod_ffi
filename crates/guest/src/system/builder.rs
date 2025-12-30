use crate::{
    QueryBuilder, World,
    query::{QueryData, QueryFilter},
};
unsafe extern "C" {
    fn bevy_param_builder_new(world_ptr: *mut (), out_builder: *mut *mut ()) -> bool;

    fn bevy_param_builder_add_query(builder: *mut (), query_ptr: *mut ()) -> bool;

    fn bevy_param_builder_build(
        world_ptr: *mut (),
        builder: *mut (),
        out_state: *mut *mut (),
    ) -> bool;

    fn bevy_param_builder_drop(builder: *mut ());
}
pub struct ParamBuilder {
    pub(crate) ptr: *mut (),
    pub(crate) world_ptr: *mut (),
}

impl ParamBuilder {
    pub fn new(world: &mut World) -> Self {
        let mut builder_ptr: *mut () = std::ptr::null_mut();

        let success = unsafe { bevy_param_builder_new(world.ptr, &mut builder_ptr) };

        if !success || builder_ptr.is_null() {
            panic!("Failed to create host-side ParamBuilder");
        }

        Self {
            ptr: builder_ptr,
            world_ptr: world.ptr,
        }
    }

    pub fn add_query<D: QueryData + 'static, F: QueryFilter + 'static>(
        &mut self,
        world: &mut World,
    ) {
        let query_builder = QueryBuilder::<D, F>::new(world);
        let query_ptr = query_builder.ptr;
        std::mem::forget(query_builder);

        let success = unsafe { bevy_param_builder_add_query(self.ptr, query_ptr) };

        if !success {
            panic!("Failed to add query to param builder");
        }
    }

    pub fn build(mut self) -> *mut () {
        let mut state_ptr: *mut () = std::ptr::null_mut();

        let success = unsafe { bevy_param_builder_build(self.world_ptr, self.ptr, &mut state_ptr) };

        self.ptr = std::ptr::null_mut();

        if !success || state_ptr.is_null() {
            panic!("Failed to build SystemState from ParamBuilder");
        }

        state_ptr
    }
}

impl Drop for ParamBuilder {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { bevy_param_builder_drop(self.ptr) };
        }
    }
}

pub struct ParamCursor<'a> {
    data: &'a [*mut ()],
    position: usize,
}

impl<'a> ParamCursor<'a> {
    pub fn new(data: &'a [*mut ()]) -> Self {
        Self { data, position: 0 }
    }

    pub fn next(&mut self) -> Option<*mut ()> {
        if self.position < self.data.len() {
            let ptr = self.data[self.position];
            self.position += 1;
            Some(ptr)
        } else {
            None
        }
    }

    pub fn position(&self) -> usize {
        self.position
    }
}
