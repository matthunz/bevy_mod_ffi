pub use bevy_mod_ffi_host_sys as sys;

use bevy::ecs::world::World;
use libloading::{Library, Symbol};
use std::{error::Error, ffi::OsStr};

pub mod query;

pub mod system;

pub mod world;

pub use bevy_mod_ffi_host_sys::SharedRegistry;

pub unsafe fn run(path: impl AsRef<OsStr>, world: &mut World) -> Result<(), Box<dyn Error>> {
    let guest_lib = unsafe { Library::new(path)? };

    let main_fn: Symbol<unsafe extern "C" fn(*mut bevy_mod_ffi_core::world)> =
        unsafe { guest_lib.get(b"bevy_main")? };
    unsafe { main_fn(world as *mut World as *mut bevy_mod_ffi_core::world) };

    Ok(())
}
