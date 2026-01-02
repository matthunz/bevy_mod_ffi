use bevy::prelude::*;
use bevy_mod_ffi::host::prelude::*;
use bevy_mod_ffi_example_core::{Damage, Position, Velocity};

fn main() {
    let mut registry = SharedRegistry::default();
    registry.register_event::<Damage>();

    let mut app = App::new();
    app.add_plugins((DefaultPlugins, FfiPlugin))
        .init_resource::<AppTypeRegistry>()
        .insert_resource(registry)
        .add_systems(Startup, setup);

    app.world_mut().register_component::<Position>();
    app.world_mut().register_component::<Velocity>();
    app.run();
}

fn setup(asset_server: Res<AssetServer>) {
    let path = format!(
        "{}bevy_mod_ffi_example_guest.{}",
        std::env::consts::DLL_PREFIX,
        std::env::consts::DLL_EXTENSION
    );
    let _ = asset_server.load::<DynamicPlugin>(path);
}
