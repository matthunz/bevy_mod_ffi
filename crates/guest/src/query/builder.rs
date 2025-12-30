use super::{QueryData, QueryFilter, QueryState};
use crate::World;
use bevy_ecs::component::ComponentId;
use bevy_reflect::TypePath;
use std::{marker::PhantomData, mem, ptr};

unsafe extern "C" {
    pub fn bevy_query_builder_new(world_ptr: *mut (), out_builder: *mut *mut ()) -> bool;

    pub fn bevy_query_builder_with_ref(builder: *mut (), component_id: usize) -> bool;

    pub fn bevy_query_builder_with_mut(builder: *mut (), component_id: usize) -> bool;

    pub fn bevy_query_builder_with(builder: *mut (), component_id: usize) -> bool;

    pub fn bevy_query_builder_without(builder: *mut (), component_id: usize) -> bool;

    pub fn bevy_query_builder_build(builder: *mut (), out_state: *mut *mut ()) -> bool;

    pub fn bevy_query_builder_drop(builder: *mut ());
}

pub struct QueryBuilder<'w, D = (), F = ()> {
    pub(crate) ptr: *mut (),
    world: &'w mut World,
    _marker: PhantomData<(D, F)>,
}

impl<'w, D: QueryData, F: QueryFilter> QueryBuilder<'w, D, F> {
    pub fn new(world: &'w mut World) -> Self {
        let mut builder_ptr: *mut () = ptr::null_mut();

        let success = unsafe { bevy_query_builder_new(world.ptr, &mut builder_ptr) };
        if !success || builder_ptr.is_null() {
            panic!("Failed to create host-side QueryBuilder");
        }

        let mut me = Self {
            ptr: builder_ptr,
            world,
            _marker: PhantomData,
        };
        D::build_query(me.transmute());
        F::filter(me.transmute());

        me
    }

    pub fn with_ref_id(&mut self, component_id: ComponentId) -> &mut Self {
        let success = unsafe { bevy_query_builder_with_ref(self.ptr, component_id.index()) };
        if !success {
            panic!("Failed to add Ref access for: {}", component_id.index());
        }

        self
    }

    pub fn with_ref<T: TypePath>(&mut self) -> &mut Self {
        let component_id = self.world.get_component_id::<T>().unwrap();
        self.with_ref_id(component_id)
    }

    pub fn with_mut_id(&mut self, component_id: ComponentId) -> &mut Self {
        let success = unsafe { bevy_query_builder_with_mut(self.ptr, component_id.index()) };
        if !success {
            panic!("Failed to add Mut access for: {}", component_id.index());
        }

        self
    }

    pub fn with_mut<T: TypePath>(&mut self) -> &mut Self {
        let component_id = self.world.get_component_id::<T>().unwrap();
        self.with_mut_id(component_id)
    }

    pub fn with_id(&mut self, component_id: ComponentId) -> &mut Self {
        let success = unsafe { bevy_query_builder_with(self.ptr, component_id.index()) };
        if !success {
            panic!("Failed to add With filter for: {}", component_id.index());
        }

        self
    }

    pub fn with<T: TypePath>(&mut self) -> &mut Self {
        let component_id = self.world.get_component_id::<T>().unwrap();
        self.with_id(component_id)
    }

    pub fn without_id(&mut self, component_id: ComponentId) -> &mut Self {
        let success = unsafe { bevy_query_builder_without(self.ptr, component_id.index()) };
        if !success {
            panic!("Failed to add Without filter for: {}", component_id.index());
        }

        self
    }

    pub fn without<T: TypePath>(&mut self) -> &mut Self {
        let component_id = self.world.get_component_id::<T>().unwrap();
        self.without_id(component_id)
    }

    pub fn build(mut self) -> QueryState<D, F> {
        let mut state_ptr: *mut () = ptr::null_mut();

        let success = unsafe { bevy_query_builder_build(self.ptr, &mut state_ptr) };

        self.ptr = ptr::null_mut();

        if !success {
            panic!("Failed to build QueryState from QueryBuilder");
        }

        QueryState::from_raw(state_ptr, D::build_state(self.world))
    }

    pub fn transmute<F2, D2>(&mut self) -> &mut QueryBuilder<'w, D2, F2> {
        unsafe { mem::transmute(self) }
    }
}

impl<D, F> Drop for QueryBuilder<'_, D, F> {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { bevy_query_builder_drop(self.ptr) }
        }
    }
}
