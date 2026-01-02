use crate::{
    component::{SharedComponent, StorageType},
    query::{QueryData, QueryFilter, QueryState},
    system::{IntoSystem, System, SystemRef, SystemState},
};
use bevy_mod_ffi_core::{BundleComponent, world};
use bevy_mod_ffi_guest_sys;
use bevy_reflect::TypePath;
use std::{
    alloc::Layout,
    ffi::CString,
    ptr::{self, NonNull},
};

pub use bevy_ecs::{
    component::ComponentId,
    entity::Entity,
    ptr::{Ptr, PtrMut},
};
pub use bytemuck::{Pod, Zeroable};

mod entity;
pub use entity::{EntityWorldMut, FilteredEntityMut};

pub struct World {
    pub(crate) ptr: *mut world,
}

impl World {
    #[doc(hidden)]
    pub unsafe fn from_ptr(ptr: *mut world) -> Self {
        Self { ptr }
    }

    pub fn register_component<C: SharedComponent>(&mut self) -> ComponentId {
        let name = C::type_path();

        let layout = Layout::new::<C>();
        let name_cstring = CString::new(name).unwrap();
        let name_bytes = name_cstring.as_bytes_with_nul();

        let mut id: usize = 0;
        let success = unsafe {
            bevy_mod_ffi_guest_sys::world::bevy_world_register_component(
                self.ptr,
                name_bytes.as_ptr(),
                name_bytes.len(),
                layout.size(),
                layout.align(),
                matches!(C::STORAGE_TYPE, StorageType::Table),
                &mut id,
            )
        };

        assert!(success, "Failed to register component: {}", name);

        ComponentId::new(id)
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
            bevy_mod_ffi_guest_sys::world::bevy_world_get_resource_id(
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

        let success = unsafe {
            bevy_mod_ffi_guest_sys::world::bevy_world_get_resource(
                self.ptr,
                id.index(),
                &mut out_ptr,
            )
        };
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
            bevy_mod_ffi_guest_sys::world::bevy_world_get_component_id(
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

    pub fn run_system<Marker, S>(&mut self, system: S)
    where
        S: IntoSystem<Marker, In = (), Out = ()>,
    {
        let r = SystemState::<<S::System as System>::Param>::new(self).build(system);
        self.run_system_ref(r);
    }

    pub fn run_system_ref<S>(&mut self, system: SystemRef<S>) {
        unsafe {
            bevy_mod_ffi_guest_sys::world::bevy_world_run_system(self.ptr, system.ptr as *mut _)
        };
    }

    pub fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityWorldMut<'_> {
        let mut components = Vec::new();
        let mut storage = Vec::new();
        bundle.bundle(self, &mut components, &mut storage);

        let mut entity_bits: u64 = 0;
        let mut entity_ptr = ptr::null_mut();
        let success = unsafe {
            bevy_mod_ffi_guest_sys::world::bevy_world_spawn(
                self.ptr,
                components.as_ptr(),
                components.len(),
                &mut entity_bits,
                &mut entity_ptr,
            )
        };
        assert!(success, "Failed to spawn entity");

        let entity = Entity::from_bits(entity_bits);
        unsafe { EntityWorldMut::from_ptr(entity, entity_ptr) }
    }
}

pub trait Bundle {
    fn bundle(
        self,
        world: &mut World,
        components: &mut Vec<BundleComponent>,
        storage: &mut Vec<Box<[u8]>>,
    );
}

impl<C: SharedComponent + Pod> Bundle for C {
    fn bundle(
        self,
        world: &mut World,
        components: &mut Vec<BundleComponent>,
        storage: &mut Vec<Box<[u8]>>,
    ) {
        let component_id = world.get_component_id::<C>().unwrap();
        let bytes = bytemuck::bytes_of(&self).to_vec().into_boxed_slice();
        let ptr = bytes.as_ptr();
        storage.push(bytes);
        components.push(BundleComponent {
            component_id: component_id.index(),
            ptr,
        });
    }
}

macro_rules! impl_bundle_tuple {
    ($($item:ident),+) => {
        impl<$($item: Bundle),+> Bundle for ($($item,)+) {
            fn bundle(self, world: &mut World, components: &mut Vec<BundleComponent>, storage: &mut Vec<Box<[u8]>>) {
                #[allow(non_snake_case)]
                let ($($item,)+) = self;
                $(
                    $item.bundle(world, components, storage);
                )+
            }
        }
    };
}

impl_bundle_tuple!(B0, B1);
impl_bundle_tuple!(B0, B1, B2);
impl_bundle_tuple!(B0, B1, B2, B3);
impl_bundle_tuple!(B0, B1, B2, B3, B4);
impl_bundle_tuple!(B0, B1, B2, B3, B4, B5);
impl_bundle_tuple!(B0, B1, B2, B3, B4, B5, B6);
impl_bundle_tuple!(B0, B1, B2, B3, B4, B5, B6, B7);
