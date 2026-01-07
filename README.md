# bevy_mod_ffi

[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/matthunz/bevy_mod_ffi#license)
[![Crates.io](https://img.shields.io/crates/v/bevy_mod_ffi.svg)](https://crates.io/crates/bevy_mod_ffi)
[![Downloads](https://img.shields.io/crates/d/bevy_mod_ffi.svg)](https://crates.io/crates/bevy_mod_ffi)
[![Docs](https://docs.rs/bevy_mod_ffi/badge.svg)](https://docs.rs/bevy_mod_ffi/latest/bevy_mod_ffi/)
[![CI](https://github.com/matthunz/bevy_mod_ffi/workflows/CI/badge.svg)](https://github.com/matthunz/bevy_mod_ffi/actions)

#### Client
```rs
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
```

#### Host
```rs
use bevy::prelude::*;
use bevy_mod_ffi::SharedRegistry;
use bevy_mod_ffi_example_core::{Damage, Health};

fn main() {
    let mut registry = SharedRegistry::default();
    registry.register_event::<Damage>();

    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_resource::<AppTypeRegistry>()
        .insert_resource(registry);

    app.world_mut().register_component::<Health>();
    app.update();

    let path = format!(
        "target/debug/{}bevy_mod_ffi_example_guest.{}",
        std::env::consts::DLL_PREFIX,
        std::env::consts::DLL_EXTENSION
    );

    let lib = unsafe { bevy_mod_ffi::run(path, app.world_mut()).unwrap() };
    lib.unload(app.world_mut());
}
```
