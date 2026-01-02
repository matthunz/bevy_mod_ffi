use bevy_mod_ffi::prelude::*;
use bevy_mod_ffi_example_core::{Damage, Position, Velocity};
use bevy_reflect::TypePath;

#[repr(C)]
#[derive(Clone, Copy, Debug, Zeroable, Pod, TypePath)]
struct Zombie;

impl SharedComponent for Zombie {}

#[bevy_mod_ffi::main]
fn main(world: &mut World) {
    world.register_component::<Zombie>();

    world.spawn((
        Zombie,
        Position { x: 0.0, y: 0.0 },
        Velocity { x: 1.0, y: 1.0 },
    ));

    world.run_system(
        (),
        |mut query: Query<(Entity, &mut Position, &Velocity), With<Zombie>>| {
            for (entity, pos, vel) in query.iter_mut() {
                dbg!(entity, &pos, vel);

                pos.x += vel.x;
                pos.y += vel.y;
            }
        },
    );

    world.add_observer(|event: On<Damage>| {
        println!("Ouch! Amount: {}", event.amount);
    });

    world.trigger(Damage { amount: 42.0 });
}
