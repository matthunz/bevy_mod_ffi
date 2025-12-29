use crate::query::{QueryData, QueryFilter, QueryState};
use bevy_reflect::TypePath;
use std::{ffi::CString, ptr::NonNull};

pub use bevy_ecs::{
    component::ComponentId,
    ptr::{Ptr, PtrMut},
};
pub use bytemuck::{Pod, Zeroable};

mod entity;
pub use entity::FilteredEntityMut;

pub mod query;
pub use query::{Query, QueryBuilder};

pub mod system;
pub use system::{SystemParam, SystemState};

unsafe extern "C" {
    fn bevy_world_get_resource_id(
        world: *mut (),
        type_path_ptr: *const u8,
        type_path_len: usize,
        out_id: *mut usize,
    ) -> bool;

    fn bevy_world_get_resource(world: *mut (), component_id: usize, out_ptr: *mut *mut u8) -> bool;

    fn bevy_world_get_component_id(
        world: *mut (),
        type_path_ptr: *const u8,
        type_path_len: usize,
        out_id: *mut usize,
    ) -> bool;
}

pub struct World {
    ptr: *mut (),
}

impl World {
    pub unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr }
    }

    pub fn get_resource_id<R>(&self) -> Option<ComponentId>
    where
        R: TypePath,
    {
        self.get_resource_id_from_type_path(R::type_path())
    }

    pub fn get_resource_id_from_type_path(&self, type_path: &str) -> Option<ComponentId> {
        let type_path_cstring = CString::new(type_path).unwrap();
        let type_path_bytes = type_path_cstring.as_bytes_with_nul();

        let mut id: usize = 0;

        let success = unsafe {
            bevy_world_get_resource_id(
                self.ptr,
                type_path_bytes.as_ptr(),
                type_path_bytes.len(),
                &mut id,
            )
        };
        if !success {
            return None;
        }

        Some(ComponentId::new(id))
    }

    pub fn get_resource<R>(&self) -> Option<&R>
    where
        R: TypePath + Pod + Zeroable,
    {
        let id = self.get_resource_id_from_type_path(R::type_path())?;
        let ptr = self.get_resource_by_id(id)?;
        Some(unsafe { ptr.deref() })
    }

    pub fn get_resource_by_id(&self, id: ComponentId) -> Option<Ptr<'_>> {
        let mut out_ptr: *mut u8 = std::ptr::null_mut();

        let success = unsafe { bevy_world_get_resource(self.ptr, id.index(), &mut out_ptr) };
        if !success {
            return None;
        }

        let ptr = NonNull::new(out_ptr)?;
        Some(unsafe { Ptr::new(ptr) })
    }

    pub fn get_component_id<R>(&self) -> Option<ComponentId>
    where
        R: TypePath,
    {
        self.get_component_id_from_type_path(R::type_path())
    }

    pub fn get_component_id_from_type_path(&self, type_path: &str) -> Option<ComponentId> {
        let type_path_cstring = CString::new(type_path).unwrap();
        let type_path_bytes = type_path_cstring.as_bytes_with_nul();

        let mut id: usize = 0;

        let success = unsafe {
            bevy_world_get_component_id(
                self.ptr,
                type_path_bytes.as_ptr(),
                type_path_bytes.len(),
                &mut id,
            )
        };
        if !success {
            return None;
        }

        Some(ComponentId::new(id))
    }

    pub fn query<D: QueryData>(&mut self) -> QueryState<D> {
        self.query_filtered()
    }

    pub fn query_filtered<D: QueryData, F: QueryFilter>(&mut self) -> QueryState<D, F> {
        QueryState::new(self)
    }
}
