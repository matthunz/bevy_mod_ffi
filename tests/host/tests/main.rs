use bevy::prelude::*;
use bevy_mod_ffi::SharedRegistry;
use bevy_mod_ffi_test_core::{Counter, TestMarker};

fn get_guest_library_path() -> String {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
    let workspace_root = std::path::Path::new(&manifest_dir)
        .parent()
        .and_then(|p| p.parent())
        .unwrap_or(std::path::Path::new("."));

    let lib_path = workspace_root.join("target").join("debug").join(format!(
        "{}bevy_mod_ffi_test_guest.{}",
        std::env::consts::DLL_PREFIX,
        std::env::consts::DLL_EXTENSION
    ));

    lib_path.to_string_lossy().to_string()
}

fn setup_app() -> App {
    let registry = SharedRegistry::default();

    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_resource::<AppTypeRegistry>()
        .insert_resource(registry);

    app.world_mut().register_component::<Counter>();
    app.world_mut().register_component::<TestMarker>();
    app.update();

    app
}

#[test]
fn test_guest_library_loads() {
    let mut app = setup_app();
    let path = get_guest_library_path();

    let result = unsafe { bevy_mod_ffi::run(&path, app.world_mut()) };
    assert!(
        result.is_ok(),
        "Failed to load guest library: {:?}",
        result.err()
    );
}

#[test]
fn test_entities_spawned_by_guest() {
    let mut app = setup_app();
    let path = get_guest_library_path();

    unsafe { bevy_mod_ffi::run(&path, app.world_mut()).expect("Failed to load guest library") };

    let world = app.world_mut();
    let mut query = world.query::<&Counter>();
    let count = query.iter(world).count();

    assert_eq!(
        count, 3,
        "Expected 3 entities with Counter, found {}",
        count
    );
}

#[test]
fn test_component_values_modified_by_guest() {
    let mut app = setup_app();
    let path = get_guest_library_path();

    unsafe { bevy_mod_ffi::run(&path, app.world_mut()).expect("Failed to load guest library") };

    let world = app.world_mut();
    let mut query = world.query::<&Counter>();
    let values: Vec<i32> = query.iter(world).map(|c| c.value).collect();

    assert!(
        values.contains(&84),
        "Expected counter value 84 (42*2), found {:?}",
        values
    );
    assert!(
        values.contains(&200),
        "Expected counter value 200 (100*2), found {:?}",
        values
    );
}

#[test]
fn test_filter_query_works() {
    let mut app = setup_app();
    let path = get_guest_library_path();

    unsafe { bevy_mod_ffi::run(&path, app.world_mut()).expect("Failed to load guest library") };

    let world = app.world_mut();
    let mut query = world.query::<(&Counter, &TestMarker)>();
    let count = query.iter(world).count();

    assert_eq!(
        count, 1,
        "Expected 1 entity with Counter+TestMarker, found {}",
        count
    );
}
