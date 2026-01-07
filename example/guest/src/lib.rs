use bevy_mod_ffi::prelude::*;
use bevy_mod_ffi_example_core::{Damage, Health};

#[repr(C)]
#[derive(SharedComponent, Clone, Copy, Debug, Zeroable, Pod, TypePath)]
struct Zombie;

#[bevy_mod_ffi::main]
fn main(world: &mut World) {
    world.register_component::<Zombie>();

    world.spawn((Zombie, Health { current: 100.0 })).observe(
        |event: OnEntity<Damage>, mut query: Query<&mut Health>| {
            println!("Entity {:?} took {} damage!", event.entity, event.amount);

            let health = query.get_mut(event.entity).unwrap();
            health.current -= event.amount;
        },
    );
}
