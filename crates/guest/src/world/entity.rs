use crate::world::World;
use bevy_ecs::{
    component::ComponentId,
    ptr::{Ptr, PtrMut},
};
use std::{marker::PhantomData, ptr::NonNull};

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
    _marker: PhantomData<&'w mut World>,
}

impl<'w> FilteredEntityMut<'w> {
    pub(crate) unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self {
            ptr,
            _marker: PhantomData,
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
}

impl Drop for FilteredEntityMut<'_> {
    fn drop(&mut self) {
        unsafe { bevy_filtered_entity_mut_drop(self.ptr) };
    }
}
