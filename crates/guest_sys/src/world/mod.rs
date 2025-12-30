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

    pub fn bevy_world_run_system(world_ptr: *mut world, system_ptr: *mut system);
}
