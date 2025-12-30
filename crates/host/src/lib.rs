use bevy::ecs::world::World;
use libloading::{Library, Symbol};
use std::{error::Error, ffi::OsStr, path::Path};

mod query;

mod system;
pub use system::GuestRunSystemFn;

mod world;

pub type GuestRunSystemFnType = unsafe extern "C" fn(*mut (), *const (), usize);

/// Safety:
/// - `path` must be a valid path to a dynamic library compiled for the same architecture
/// and with the same version of this crate.
pub unsafe fn run(path: impl AsRef<OsStr>, world: &mut World) -> Result<(), Box<dyn Error>> {
    let guest_lib = unsafe { Library::new(path)? };

    let main_fn: Symbol<unsafe extern "C" fn(*mut ())> = unsafe { guest_lib.get(b"bevy_main")? };
    unsafe { main_fn(world as *mut World as *mut ()) };

    Ok(())
}
