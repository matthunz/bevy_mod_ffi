use bevy_mod_ffi::prelude::*;
use bevy_mod_ffi_example_core::{ExampleResource, Position, Velocity};

#[bevy_mod_ffi::main]
fn main(world: &mut World) {
    let r = world.get_resource::<ExampleResource>().unwrap();
    dbg!(r);

    let mut query = world.query::<(&Position, &mut Velocity)>();
    for (entity, (pos, vel)) in query.iter_mut(world) {
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
