use bevy::prelude::*;
use bevy_mod_ffi::DynamicComponentRegistry;
use bevy_mod_ffi_example_core::{Position, Velocity};

fn main() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_resource::<AppTypeRegistry>()
        .init_resource::<DynamicComponentRegistry>();
    app.world_mut().register_component::<Position>();
    app.world_mut().register_component::<Velocity>();
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
