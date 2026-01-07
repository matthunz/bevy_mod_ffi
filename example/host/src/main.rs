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
