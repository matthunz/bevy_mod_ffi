use bevy_mod_ffi_core::*;

pub mod entity;

pub use entity::*;

unsafe extern "C" {
    pub fn bevy_world_get_resource_id(
        world: *mut world,
        type_path_ptr: *const u8,
        type_path_len: usize,
        out_id: *mut usize,
    ) -> bool;

    pub fn bevy_world_get_resource(
        world: *mut world,
        component_id: usize,
        out_ptr: *mut *mut u8,
    ) -> bool;

    pub fn bevy_world_get_component_id(
        world: *mut world,
        type_path_ptr: *const u8,
        type_path_len: usize,
        out_id: *mut usize,
    ) -> bool;

    pub fn bevy_world_run_system(
        world_ptr: *mut world,
        system_ptr: *mut system,
        input_ptr: *const u8,
        output_ptr: *mut u8,
    );

    pub fn bevy_world_register_component(
        world: *mut world,
        name_ptr: *const u8,
        name_len: usize,
        size: usize,
        align: usize,
        is_table: bool,
        out_id: *mut usize,
    ) -> bool;

    pub fn bevy_world_spawn(
        world: *mut world,
        components_ptr: *const BundleComponent,
        component_len: usize,
        out_entity: *mut u64,
        out_entity_world_mut_ptr: *mut *mut entity_world_mut,
    ) -> bool;

    pub fn bevy_world_trigger_event(
        world: *mut world,
        event_name_ptr: *const u8,
        event_name_len: usize,
        event_data_ptr: *const u8,
        event_data_len: usize,
    ) -> bool;

    pub fn bevy_world_trigger_event_targets(
        world: *mut world,
        event_name_ptr: *const u8,
        event_name_len: usize,
        event_data_ptr: *const u8,
        event_data_len: usize,
        entity_bits: u64,
    ) -> bool;
}
