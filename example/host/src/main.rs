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

    unsafe { bevy_mod_ffi_host::run(guest_lib_path, app.world_mut()).unwrap() }
}
