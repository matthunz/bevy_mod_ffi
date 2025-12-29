use crate::World;
use bevy_ecs::{
    component::ComponentId,
    ptr::{Ptr, PtrMut},
};
use bevy_reflect::TypePath;
use bytemuck::Pod;
use std::ptr::NonNull;

unsafe extern "C" {
    pub fn bevy_filtered_entity_mut_get_component(
        entity: *mut (),
        component_id: usize,
        out_ptr: *mut *mut u8,
    ) -> bool;

    pub fn bevy_filtered_entity_mut_get_component_mut(
        entity: *mut (),
        component_id: usize,
        out_ptr: *mut *mut u8,
    ) -> bool;

    pub fn bevy_filtered_entity_mut_drop(entity: *mut ());
}

pub struct FilteredEntityMut<'w> {
    ptr: *mut (),
    // TODO change to unsafe cell
    world: &'w mut World,
}

impl<'w> FilteredEntityMut<'w> {
    pub(crate) unsafe fn from_ptr(entity_ptr: *mut (), world: &'w mut World) -> Self {
        Self {
            ptr: entity_ptr,
            world,
        }
    }

    pub fn get_by_id(&self, component_id: ComponentId) -> Option<Ptr<'w>> {
        let mut out_ptr = std::ptr::null_mut();

        let success = unsafe {
            bevy_filtered_entity_mut_get_component(self.ptr, component_id.index(), &mut out_ptr)
        };
        if !success {
            return None;
        }

        let ptr = NonNull::new(out_ptr)?;
        Some(unsafe { Ptr::new(ptr) })
    }

    pub fn get<T: Pod + TypePath>(&self) -> Option<&'w T> {
        let id = self.world.get_component_id::<T>()?;
        let ptr = self.get_by_id(id)?;
        Some(unsafe { ptr.deref() })
    }

    pub fn get_mut_by_id(&mut self, component_id: ComponentId) -> Option<PtrMut<'w>> {
        let mut out_ptr = std::ptr::null_mut();

        let success = unsafe {
            bevy_filtered_entity_mut_get_component_mut(self.ptr, component_id.index(), &mut out_ptr)
        };
        if !success || out_ptr.is_null() {
            return None;
        }

        let ptr = NonNull::new(out_ptr)?;
        Some(unsafe { PtrMut::new(ptr) })
    }

    pub fn get_mut<T: Pod + TypePath>(&mut self) -> Option<&'w mut T> {
        let id = self.world.get_component_id::<T>()?;
        let ptr_mut = self.get_mut_by_id(id)?;
        Some(unsafe { ptr_mut.deref_mut() })
    }
}

impl Drop for FilteredEntityMut<'_> {
    fn drop(&mut self) {
        unsafe { bevy_filtered_entity_mut_drop(self.ptr) };
    }
}
