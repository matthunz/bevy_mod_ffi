use crate::{
    component::{HookContext, SharedComponent, StorageType},
    query::{QueryData, QueryFilter, QueryState},
    system::{
        IntoObserverSystem, IntoSystem, On, ParamBuilder, ParamCursor, SharedEvent, System,
        SystemParam, SystemRef, SystemState,
    },
};
use bevy_mod_ffi_core::{
    BundleComponent, ComponentHookFn, deferred_world, dyn_system_param, trigger, world,
};
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

mod deferred;
pub use deferred::DeferredWorld;

macro_rules! make_hook_wrapper {
    ($name:ident, $hook_getter:expr) => {
        unsafe extern "C" fn $name<C: SharedComponent>(
            deferred_ptr: *mut deferred_world,
            entity_bits: u64,
            component_id: usize,
        ) {
            if let Some(hook) = $hook_getter {
                let deferred = unsafe { DeferredWorld::from_ptr(deferred_ptr) };
                let context = HookContext {
                    entity: Entity::from_bits(entity_bits),
                    component_id: ComponentId::new(component_id),
                    caller: None,
                };
                hook(deferred, context);
            }
        }
    };
}

make_hook_wrapper!(on_add_wrapper, C::on_add());
make_hook_wrapper!(on_insert_wrapper, C::on_insert());
make_hook_wrapper!(on_replace_wrapper, C::on_replace());
make_hook_wrapper!(on_remove_wrapper, C::on_remove());
make_hook_wrapper!(on_despawn_wrapper, C::on_despawn());

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

        let on_add: Option<ComponentHookFn> =
            C::on_add().map(|_| on_add_wrapper::<C> as ComponentHookFn);
        let on_insert: Option<ComponentHookFn> =
            C::on_insert().map(|_| on_insert_wrapper::<C> as ComponentHookFn);
        let on_replace: Option<ComponentHookFn> =
            C::on_replace().map(|_| on_replace_wrapper::<C> as ComponentHookFn);
        let on_remove: Option<ComponentHookFn> =
            C::on_remove().map(|_| on_remove_wrapper::<C> as ComponentHookFn);
        let on_despawn: Option<ComponentHookFn> =
            C::on_despawn().map(|_| on_despawn_wrapper::<C> as ComponentHookFn);

        let mut id: usize = 0;
        let success = unsafe {
            bevy_mod_ffi_guest_sys::world::bevy_world_register_component(
                self.ptr,
                name_bytes.as_ptr(),
                name_bytes.len(),
                layout.size(),
                layout.align(),
                matches!(C::STORAGE_TYPE, StorageType::Table) as u8,
                on_add,
                on_insert,
                on_replace,
                on_remove,
                on_despawn,
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

    pub fn run_system<Marker, In, Out, S>(&mut self, input: In, system: S) -> Out
    where
        S: IntoSystem<Marker, In = In, Out = Out>,
        S::System: 'static,
        <S::System as System>::Param: 'static,
        In: Pod,
        Out: Pod,
    {
        let r = SystemState::<<S::System as System>::Param>::new(self).build(system);
        self.run_system_ref(input, r)
    }

    pub fn run_system_ref<In, Out, S>(&mut self, input: In, system: SystemRef<S>) -> Out
    where
        In: Pod,
        Out: Pod,
    {
        let input_bytes = bytemuck::bytes_of(&input);
        let mut output = bytemuck::zeroed_box::<Out>();

        unsafe {
            bevy_mod_ffi_guest_sys::world::bevy_world_run_system(
                self.ptr,
                system.ptr as *mut _,
                input_bytes.as_ptr(),
                &mut *output as *mut _ as _,
            )
        };

        *output
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
        unsafe { EntityWorldMut::from_ptr(entity, entity_ptr, self) }
    }

    pub fn entity_mut(&mut self, entity: Entity) -> EntityWorldMut<'_> {
        let mut entity_ptr = ptr::null_mut();
        let success = unsafe {
            bevy_mod_ffi_guest_sys::world::bevy_world_entity_mut(
                self.ptr,
                entity.to_bits(),
                &mut entity_ptr,
            )
        };
        assert!(success, "Failed to get entity {:?}", entity);

        unsafe { EntityWorldMut::from_ptr(entity, entity_ptr, self) }
    }

    pub fn add_observer<E, Marker, S>(&mut self, observer: S)
    where
        E: SharedEvent + 'static,
        S: IntoObserverSystem<E, Marker>,
        S::System: 'static,
        <S::System as System>::Param: 'static,
    {
        let mut system = observer.into_system();
        let mut builder = ParamBuilder::new(self);
        let mut state = <<S::System as System>::Param as SystemParam>::build(self, &mut builder);
        let state_ptr = builder.build();

        let event_name = E::type_path();
        let event_name_cstring = CString::new(event_name).unwrap();
        let event_name_bytes = event_name_cstring.as_bytes_with_nul();

        type ObserverClosure = Box<dyn FnMut(&[*mut dyn_system_param], *mut trigger)>;

        let observer_boxed: ObserverClosure = Box::new(move |params, event_ptr| {
            let mut param_cursor = ParamCursor::new(params);
            let params = unsafe {
                <<S::System as System>::Param as SystemParam>::get_param(
                    &mut state,
                    &mut param_cursor,
                )
            };

            let event = unsafe { &*(event_ptr as *const E) };
            system.run(On { event }, params);
        });

        let success = unsafe {
            bevy_mod_ffi_guest_sys::system::bevy_system_state_build_on(
                self.ptr,
                state_ptr,
                event_name_bytes.as_ptr(),
                event_name_bytes.len(),
                Box::into_raw(Box::new(observer_boxed)) as _,
                bevy_mod_ffi_guest_sys::system::bevy_guest_run_observer,
            )
        };

        assert!(success, "Failed to add observer for event: {}", event_name);
    }

    pub fn trigger<E: SharedEvent>(&mut self, event: E) {
        let event_name = E::type_path();
        let event_name_cstring = CString::new(event_name).unwrap();
        let event_name_bytes = event_name_cstring.as_bytes_with_nul();
        let event_bytes = bytemuck::bytes_of(&event);

        let success = unsafe {
            bevy_mod_ffi_guest_sys::world::bevy_world_trigger_event(
                self.ptr,
                event_name_bytes.as_ptr(),
                event_name_bytes.len(),
                event_bytes.as_ptr(),
                event_bytes.len(),
            )
        };

        assert!(success, "Failed to trigger event: {}", event_name);
    }

    pub fn trigger_targets<E: SharedEvent>(&mut self, event: E, entity: bevy_ecs::entity::Entity) {
        let event_name = E::type_path();
        let event_name_cstring = CString::new(event_name).unwrap();
        let event_name_bytes = event_name_cstring.as_bytes_with_nul();
        let event_bytes = bytemuck::bytes_of(&event);

        let success = unsafe {
            bevy_mod_ffi_guest_sys::world::bevy_world_trigger_event_targets(
                self.ptr,
                event_name_bytes.as_ptr(),
                event_name_bytes.len(),
                event_bytes.as_ptr(),
                event_bytes.len(),
                entity.to_bits(),
            )
        };

        assert!(
            success,
            "Failed to trigger event for entity: {}",
            event_name
        );
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
