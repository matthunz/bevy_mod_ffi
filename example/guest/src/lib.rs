use bevy_mod_ffi_example_core::{ExampleResource, Position, Velocity};
use bevy_mod_ffi_guest::{Query, SystemState, World};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn guest_main(world_ptr: *mut ()) {
    let mut world = unsafe { World::from_ptr(world_ptr) };

    let r = world.get_resource::<ExampleResource>().unwrap();
    dbg!(r);

    let mut query = world.query::<(&Position, &mut Velocity)>();
    for (entity, (pos, vel)) in query.iter_mut(&mut world) {
        dbg!(entity, pos, &vel);

        vel.x *= 2.0;
        vel.y *= 2.0;
    }

    let mut state = SystemState::<Query<&Velocity>>::new(&mut world);
    let mut query = state.get(&mut world);
    for x in query.iter_mut() {
        dbg!(x);
    }
}
