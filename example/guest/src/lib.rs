use bevy_mod_ffi_example_core::{ExampleResource, Position, Velocity};
use bevy_mod_ffi_guest::{Query, World};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_main(world_ptr: *mut ()) {
    let mut world = unsafe { World::from_ptr(world_ptr) };

    let r = world.get_resource::<ExampleResource>().unwrap();
    dbg!(r);

    let mut query = world.query::<(&Position, &mut Velocity)>();
    for (entity, (pos, vel)) in query.iter_mut(&mut world) {
        dbg!(entity, pos, &vel);

        vel.x *= 2.0;
        vel.y *= 2.0;
    }

    world.run_system(|mut query: Query<&Velocity>| {
        for x in query.iter_mut() {
            dbg!(x);
        }
    });
}
