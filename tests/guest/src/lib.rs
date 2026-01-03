use bevy_mod_ffi::prelude::*;
use bevy_mod_ffi_test_core::{Counter, TestMarker};
use bevy_reflect::TypePath;

#[repr(C)]
#[derive(Clone, Copy, Debug, Zeroable, Pod, TypePath)]
struct GuestMarker;

impl SharedComponent for GuestMarker {
    type Mutability = Mutable;
    const STORAGE_TYPE: StorageType = StorageType::Table;
}

#[bevy_mod_ffi::main]
fn main(world: &mut World) {
    world.register_component::<GuestMarker>();

    world.spawn((GuestMarker, Counter { value: 42 }, TestMarker));

    world.spawn((GuestMarker, Counter { value: 100 }));

    world.run_system((), |mut query: Query<&mut Counter, With<GuestMarker>>| {
        for counter in query.iter_mut() {
            counter.value *= 2;
        }
    });

    world.run_system((), |mut query: Query<&Counter, With<TestMarker>>| {
        let count = query.iter_mut().count();
        assert_eq!(
            count, 1,
            "Expected 1 entity with TestMarker, found {}",
            count
        );
    });

    world.spawn((GuestMarker, Counter { value: 0 }));
}
