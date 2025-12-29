use bevy_mod_ffi_example_core::ExampleResource;
use bevy_mod_ffi_guest::World;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn guest_main(world_ptr: *mut ()) {
    let world = unsafe { World::from_ptr(world_ptr) };

    let r = world.get_resource::<ExampleResource>().unwrap();
    dbg!(r.value);
}
