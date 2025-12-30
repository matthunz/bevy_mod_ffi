use bevy::ecs::world::World;
use libloading::{Library, Symbol};
use std::error::Error;

mod query;

mod system;
pub use system::GuestRunSystemFn;

mod world;

pub type GuestRunSystemFnType = unsafe extern "C" fn(*mut (), *const (), usize);

pub unsafe fn run(guest_lib_path: &str, world: &mut World) -> Result<(), Box<dyn Error>> {
    let guest_lib = unsafe { Library::new(guest_lib_path)? };

    let main_fn: Symbol<unsafe extern "C" fn(*mut ())> = unsafe { guest_lib.get(b"bevy_main")? };
    unsafe { main_fn(world as *mut World as *mut ()) };

    Ok(())
}
