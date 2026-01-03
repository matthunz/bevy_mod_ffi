#![allow(non_camel_case_types)]

/// A component to be inserted into an entity.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BundleComponent {
    pub component_id: usize,
    pub ptr: *const u8,
}

/// Opaque type for World pointers.
pub enum world {}

/// Opaque type for QueryBuilder pointers.
pub enum query_builder {}

/// Opaque type for QueryState pointers.
pub enum query_state {}

/// Opaque type for Query pointers.
pub enum query {}

/// Opaque type for QueryIter pointers.
pub enum query_iter {}

/// Opaque type for EntityWorldMut pointers.
pub enum entity_world_mut {}

/// Opaque type for FilteredEntityMut pointers.
pub enum filtered_entity_mut {}

/// Opaque type for ParamBuilder pointers.
pub enum param_builder {}

/// Opaque type for SystemState pointers.
pub enum system_state {}

/// Opaque type for DynSystemParams pointers.
pub enum dyn_system_param {}

/// Opaque type for System pointers.
pub enum system {}

/// Opaque type for Observer pointers.
pub enum observer {}

/// Opaque type for DynamicEvent pointers.
pub enum dynamic_event {}

/// Opaque type for Trigger pointers.
pub enum trigger {}

/// Opaque type for Commands pointers.
pub enum commands {}

/// Opaque type for DeferredWorld pointers.
pub enum deferred_world {}

pub type RunSystemFn =
    unsafe extern "C" fn(*mut (), *const *mut dyn_system_param, usize, *const u8, *mut u8);

pub type RunObserverFn =
    unsafe extern "C" fn(*mut (), *const *mut dyn_system_param, usize, *mut trigger);

pub type RunCommandFn = unsafe extern "C" fn(*mut (), *mut world);

pub type ComponentHookFn = unsafe extern "C" fn(*mut deferred_world, u64, usize);
