# bevy_mod_ffi

[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/matthunz/bevy_mod_ffi#license)
[![Crates.io](https://img.shields.io/crates/v/bevy_mod_ffi.svg)](https://crates.io/crates/bevy_mod_ffi)
[![Downloads](https://img.shields.io/crates/d/bevy_mod_ffi.svg)](https://crates.io/crates/bevy_mod_ffi)
[![Docs](https://docs.rs/bevy_mod_ffi/badge.svg)](https://docs.rs/bevy_mod_ffi/latest/bevy_mod_ffi/)
[![CI](https://github.com/matthunz/bevy_mod_ffi/workflows/CI/badge.svg)](https://github.com/matthunz/bevy_mod_ffi/actions)

#### Client
```rs
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
```

#### Host
```rs
use bevy::prelude::*;
use bevy_mod_ffi::SharedRegistry;
use bevy_mod_ffi_example_core::{Damage, Position, Velocity};

fn main() {
    let mut registry = SharedRegistry::default();
    registry.register_event::<Damage>();

    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_resource::<AppTypeRegistry>()
        .insert_resource(registry);

    app.world_mut().register_component::<Position>();
    app.world_mut().register_component::<Velocity>();
    app.update();

    let path = format!(
        "target/debug/{}bevy_mod_ffi_example_guest.{}",
        std::env::consts::DLL_PREFIX,
        std::env::consts::DLL_EXTENSION
    );
    unsafe { bevy_mod_ffi::run(path, app.world_mut()).unwrap() }
}
```
