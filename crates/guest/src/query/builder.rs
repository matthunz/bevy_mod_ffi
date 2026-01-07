use super::{QueryData, QueryFilter, QueryState};
use crate::world::World;
use bevy_ecs::component::ComponentId;
use bevy_mod_ffi_core::query_builder;
use bevy_mod_ffi_guest_sys;
use bevy_reflect::TypePath;
use std::{marker::PhantomData, mem};

pub struct QueryBuilder<'w, D = (), F = ()> {
    pub(crate) ptr: *mut query_builder,
    world: &'w mut World,
    _marker: PhantomData<(D, F)>,
}

impl<'w, D: QueryData, F: QueryFilter> QueryBuilder<'w, D, F> {
    pub fn new(world: &'w mut World) -> Self {
        let ptr =
            unsafe { bevy_mod_ffi_guest_sys::query::builder::bevy_query_builder_new(world.ptr) };

        let mut me = Self {
            ptr,
            world,
            _marker: PhantomData,
        };

        D::build_query(me.transmute());
        F::filter(me.transmute());

        me
    }

    pub fn with_ref_id(&mut self, component_id: ComponentId) -> &mut Self {
        unsafe {
            bevy_mod_ffi_guest_sys::query::builder::bevy_query_builder_with_ref(
                self.ptr,
                component_id.index(),
            )
        };

        self
    }

    pub fn with_ref<T: TypePath>(&mut self) -> &mut Self {
        let component_id = self.world.get_component_id::<T>().unwrap();
        self.with_ref_id(component_id)
    }

    pub fn with_mut_id(&mut self, component_id: ComponentId) -> &mut Self {
        unsafe {
            bevy_mod_ffi_guest_sys::query::builder::bevy_query_builder_with_mut(
                self.ptr,
                component_id.index(),
            )
        };

        self
    }

    pub fn with_mut<T: TypePath>(&mut self) -> &mut Self {
        let component_id = self.world.get_component_id::<T>().unwrap();
        self.with_mut_id(component_id)
    }

    pub fn with_id(&mut self, component_id: ComponentId) -> &mut Self {
        unsafe {
            bevy_mod_ffi_guest_sys::query::builder::bevy_query_builder_with(
                self.ptr,
                component_id.index(),
            )
        };

        self
    }

    pub fn with<T: TypePath>(&mut self) -> &mut Self {
        let component_id = self.world.get_component_id::<T>().unwrap();
        self.with_id(component_id)
    }

    pub fn without_id(&mut self, component_id: ComponentId) -> &mut Self {
        unsafe {
            bevy_mod_ffi_guest_sys::query::builder::bevy_query_builder_without(
                self.ptr,
                component_id.index(),
            )
        };

        self
    }

    pub fn without<T: TypePath>(&mut self) -> &mut Self {
        let component_id = self.world.get_component_id::<T>().unwrap();
        self.without_id(component_id)
    }

    pub fn build(self) -> QueryState<D, F> {
        let ptr =
            unsafe { bevy_mod_ffi_guest_sys::query::builder::bevy_query_builder_build(self.ptr) };

        QueryState::from_raw(ptr, D::build_state(self.world))
    }

    pub fn transmute<F2, D2>(&mut self) -> &mut QueryBuilder<'w, D2, F2> {
        unsafe { mem::transmute(self) }
    }
}

impl<D, F> Drop for QueryBuilder<'_, D, F> {
    fn drop(&mut self) {
        unsafe { bevy_mod_ffi_guest_sys::query::builder::bevy_query_builder_drop(self.ptr) }
    }
}
