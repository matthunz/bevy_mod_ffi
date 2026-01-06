use bevy_mod_ffi::prelude::*;
use bevy_mod_ffi_example_core::{Damage, Position, Velocity};

#[repr(C)]
#[derive(SharedComponent, Clone, Copy, Debug, Zeroable, Pod, TypePath)]
struct Zombie;

#[bevy_mod_ffi::main]
fn main(world: &mut World) {
    world.register_component::<Zombie>();

    world
        .spawn((
            Zombie,
            Position { x: 0.0, y: 0.0 },
            Velocity { x: 1.0, y: 1.0 },
        ))
        .observe(|event: OnEntity<Damage>, query: Query<&Position>| {
            println!("Entity {:?} took {} damage!", event.entity, event.amount);
        });

    dbg!("done!");
}
