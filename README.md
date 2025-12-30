# bevy_mod_ffi

[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/matthunz/bevy_mod_ffi#license)
[![Crates.io](https://img.shields.io/crates/v/bevy_mod_ffi.svg)](https://crates.io/crates/bevy_mod_ffi)
[![Downloads](https://img.shields.io/crates/d/bevy_mod_ffi.svg)](https://crates.io/crates/bevy_mod_ffi)
[![Docs](https://docs.rs/bevy_mod_ffi/badge.svg)](https://docs.rs/bevy_mod_ffi/latest/bevy_mod_ffi/)
[![CI](https://github.com/matthunz/bevy_mod_ffi/workflows/CI/badge.svg)](https://github.com/matthunz/bevy_mod_ffi/actions)

#### Client
```rs
use bevy_mod_ffi::prelude::*;
use bevy_mod_ffi_example_core::{ExampleResource, Position, Velocity};

#[bevy_mod_ffi::main]
fn main(world: &mut World) {
    let r = world.get_resource::<ExampleResource>().unwrap();
    dbg!(r);

    let mut query = world.query::<(Entity, &Position, &mut Velocity)>();
    for (entity, pos, vel) in query.iter_mut(world) {
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
```

#### Host
```rs
use bevy::prelude::*;
use bevy_mod_ffi_example_core::{ExampleResource, Position, Velocity};

fn main() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.init_resource::<AppTypeRegistry>();
    app.register_type::<ExampleResource>();
    app.register_type::<Position>();
    app.register_type::<Velocity>();

    app.insert_resource(ExampleResource { value: 42 });
    app.world_mut()
        .spawn((Position { x: 1.0, y: 2.0 }, Velocity { x: 0.5, y: 0.5 }));

    app.update();

    let guest_lib_path = if cfg!(windows) {
        "target/debug/bevy_mod_ffi_example_guest.dll"
    } else if cfg!(target_os = "macos") {
        "target/debug/libbevy_mod_ffi_example_guest.dylib"
    } else {
        "target/debug/libbevy_mod_ffi_example_guest.so"
    };

    unsafe { bevy_mod_ffi::run(guest_lib_path, app.world_mut()).unwrap() }
}
```
