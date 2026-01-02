use crate::world::World;
use bevy_ecs::{
    component::ComponentId,
    entity::Entity,
    ptr::{Ptr, PtrMut},
};
use bevy_mod_ffi_core::{entity_world_mut, filtered_entity_mut};
use bevy_mod_ffi_guest_sys;
use std::{marker::PhantomData, ptr::NonNull};

pub struct EntityWorldMut<'w> {
    id: Entity,
    ptr: *mut entity_world_mut,
    _marker: PhantomData<&'w mut World>,
}

impl<'w> EntityWorldMut<'w> {
    pub(crate) unsafe fn from_ptr(id: Entity, ptr: *mut entity_world_mut) -> Self {
        Self {
            id,
            ptr,
            _marker: PhantomData,
        }
    }

    pub fn id(&self) -> Entity {
        self.id
    }
}

impl Drop for EntityWorldMut<'_> {
    fn drop(&mut self) {
        unsafe { bevy_mod_ffi_guest_sys::world::entity::bevy_world_entity_mut_drop(self.ptr) }
    }
}

pub struct FilteredEntityMut<'w> {
    id: Entity,
    ptr: *mut filtered_entity_mut,
    _marker: PhantomData<&'w mut World>,
}

impl<'w> FilteredEntityMut<'w> {
    pub(crate) unsafe fn from_ptr(id: Entity, ptr: *mut filtered_entity_mut) -> Self {
        Self {
            id,
            ptr,
            _marker: PhantomData,
        }
    }

    pub fn id(&self) -> Entity {
        self.id
    }

    pub fn get_by_id(&self, component_id: ComponentId) -> Option<Ptr<'w>> {
        let mut out_ptr = std::ptr::null_mut();

        let success = unsafe {
            bevy_mod_ffi_guest_sys::world::entity::bevy_filtered_entity_mut_get_component(
                self.ptr,
                component_id.index(),
                &mut out_ptr,
            )
        };
        if !success {
            return None;
        }

        let ptr = NonNull::new(out_ptr)?;
        Some(unsafe { Ptr::new(ptr) })
    }

    pub fn get_mut_by_id(&mut self, component_id: ComponentId) -> Option<PtrMut<'w>> {
        let mut out_ptr = std::ptr::null_mut();

        let success = unsafe {
            bevy_mod_ffi_guest_sys::world::entity::bevy_filtered_entity_mut_get_component_mut(
                self.ptr,
                component_id.index(),
                &mut out_ptr,
            )
        };
        if !success || out_ptr.is_null() {
            return None;
        }

        let ptr = NonNull::new(out_ptr)?;
        Some(unsafe { PtrMut::new(ptr) })
    }
}

impl Drop for FilteredEntityMut<'_> {
    fn drop(&mut self) {
        unsafe { bevy_mod_ffi_guest_sys::world::entity::bevy_filtered_entity_mut_drop(self.ptr) };
    }
}
