use crate::{
    query::{QueryBuilder, QueryData, QueryFilter},
    world::World,
};
use bevy_mod_ffi_core::{dyn_system_param, param_builder, system_state, world};
use bevy_mod_ffi_guest_sys;
use std::{mem, ptr};

pub struct ParamBuilder {
    pub(crate) ptr: *mut param_builder,
    pub(crate) world_ptr: *mut world,
}

impl ParamBuilder {
    pub fn new(world: &mut World) -> Self {
        let mut builder_ptr: *mut param_builder = ptr::null_mut();

        let success = unsafe {
            bevy_mod_ffi_guest_sys::system::param::bevy_param_builder_new(
                world.ptr,
                &mut builder_ptr,
            )
        };

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
        mem::forget(query_builder);

        let success = unsafe {
            bevy_mod_ffi_guest_sys::system::param::bevy_param_builder_add_query(self.ptr, query_ptr)
        };

        if !success {
            panic!("Failed to add query to param builder");
        }
    }

    pub fn add_commands(&mut self) {
        let success = unsafe {
            bevy_mod_ffi_guest_sys::system::param::bevy_param_builder_add_commands(self.ptr)
        };

        if !success {
            panic!("Failed to add commands to param builder");
        }
    }

    pub fn add_deferred_world(&mut self) {
        let success = unsafe {
            bevy_mod_ffi_guest_sys::system::param::bevy_param_builder_add_deferred_world(self.ptr)
        };

        if !success {
            panic!("Failed to add deferred world to param builder");
        }
    }

    pub fn build(self) -> *mut system_state {
        let mut state_ptr: *mut system_state = ptr::null_mut();

        let success = unsafe {
            bevy_mod_ffi_guest_sys::system::param::bevy_param_builder_build(
                self.world_ptr,
                self.ptr,
                &mut state_ptr,
            )
        };
        if !success {
            panic!("Failed to build SystemState from ParamBuilder");
        }

        mem::forget(self);
        state_ptr
    }
}

impl Drop for ParamBuilder {
    fn drop(&mut self) {
        unsafe { bevy_mod_ffi_guest_sys::system::param::bevy_param_builder_drop(self.ptr) };
    }
}

pub struct ParamCursor<'a> {
    data: &'a [*mut dyn_system_param],
    position: usize,
}

impl<'a> ParamCursor<'a> {
    pub fn new(data: &'a [*mut dyn_system_param]) -> Self {
        Self { data, position: 0 }
    }

    pub fn position(&self) -> usize {
        self.position
    }
}

impl Iterator for ParamCursor<'_> {
    type Item = *mut dyn_system_param;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position >= self.data.len() {
            None
        } else {
            let item = self.data[self.position];
            self.position += 1;
            Some(item)
        }
    }
}
