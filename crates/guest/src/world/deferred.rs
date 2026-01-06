use crate::{
    query::{Query, QueryData, QueryFilter, QueryState},
    system::{ParamBuilder, ParamCursor, SystemParam},
    world::World,
};
use bevy_ecs::entity::Entity;
use bevy_mod_ffi_core::{deferred_world, query};
use bevy_mod_ffi_guest_sys;
use std::{marker::PhantomData, ptr};

pub struct DeferredWorld<'w> {
    ptr: *mut deferred_world,
    _marker: PhantomData<&'w mut ()>,
}

impl<'w> DeferredWorld<'w> {
    pub(crate) unsafe fn from_ptr(ptr: *mut deferred_world) -> Self {
        Self {
            ptr,
            _marker: PhantomData,
        }
    }

    pub fn query<'s, D, F>(&mut self, state: &'s mut QueryState<D, F>) -> Query<'_, 's, D, F>
    where
        D: QueryData + 'static,
        F: QueryFilter + 'static,
    {
        let mut query_ptr: *mut query = ptr::null_mut();

        let success = unsafe {
            bevy_mod_ffi_guest_sys::world::deferred::bevy_deferred_world_query(
                self.ptr,
                state.as_ptr(),
                &mut query_ptr,
            )
        };

        if !success || query_ptr.is_null() {
            panic!("Failed to create query from DeferredWorld");
        }

        Query::new(query_ptr, &mut state.state)
    }

    pub fn get_mut<T: bytemuck::Pod>(
        &mut self,
        entity: Entity,
        component_id: usize,
    ) -> Option<&mut T> {
        let mut ptr: *mut u8 = ptr::null_mut();

        let success = unsafe {
            bevy_mod_ffi_guest_sys::world::deferred::bevy_deferred_world_get_mut(
                self.ptr,
                entity.to_bits(),
                component_id,
                &mut ptr,
            )
        };

        if !success || ptr.is_null() {
            return None;
        }

        Some(unsafe { &mut *(ptr as *mut T) })
    }

    pub fn get_resource_mut<T: bytemuck::Pod>(&mut self, component_id: usize) -> Option<&mut T> {
        let mut ptr: *mut u8 = ptr::null_mut();

        let success = unsafe {
            bevy_mod_ffi_guest_sys::world::deferred::bevy_deferred_world_get_resource_mut(
                self.ptr,
                component_id,
                &mut ptr,
            )
        };

        if !success || ptr.is_null() {
            return None;
        }

        Some(unsafe { &mut *(ptr as *mut T) })
    }
}

impl Drop for DeferredWorld<'_> {
    fn drop(&mut self) {
        unsafe { bevy_mod_ffi_guest_sys::world::deferred::bevy_deferred_world_drop(self.ptr) }
    }
}

unsafe impl SystemParam for DeferredWorld<'_> {
    type State = ();
    type Item<'w, 's> = DeferredWorld<'w>;

    fn build(_world: &mut World, builder: &mut ParamBuilder) {
        builder.add_deferred_world();
    }

    unsafe fn get_param<'w, 's>(
        _state: &'s mut Self::State,
        cursor: &mut ParamCursor<'_>,
    ) -> Self::Item<'w, 's> {
        let dyn_param_ptr = cursor.next().unwrap();
        let mut deferred_ptr: *mut deferred_world = ptr::null_mut();
        let success = unsafe {
            bevy_mod_ffi_guest_sys::system::param::bevy_dyn_system_param_downcast_deferred_world(
                dyn_param_ptr,
                &mut deferred_ptr,
            )
        };

        if !success || deferred_ptr.is_null() {
            panic!("Failed to downcast DynSystemParam to DeferredWorld");
        }

        unsafe { DeferredWorld::from_ptr(deferred_ptr) }
    }
}
