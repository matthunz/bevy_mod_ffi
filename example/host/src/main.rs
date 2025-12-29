use bevy::prelude::*;
use bevy_mod_ffi_example_core::ExampleResource;
use libloading::{Library, Symbol};

fn main() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.init_resource::<AppTypeRegistry>();
    app.register_type::<ExampleResource>();
    app.insert_resource(ExampleResource { value: 42 });
    app.update();

    let guest_lib_path = if cfg!(windows) {
        "target/debug/bevy_mod_ffi_example_guest.dll"
    } else if cfg!(target_os = "macos") {
        "target/debug/libbevy_mod_ffi_example_guest.dylib"
    } else {
        "target/debug/libbevy_mod_ffi_example_guest.so"
    };

    unsafe {
        let lib = Library::new(guest_lib_path).expect("Failed to load guest library");

        let guest_main: Symbol<unsafe extern "C" fn(*mut ())> =
            lib.get(b"guest_main").expect("Failed to find guest_main");

        guest_main(app.world_mut() as *mut World as *mut ());
    }
}
