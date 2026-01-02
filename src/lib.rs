pub use bevy_mod_ffi_core;

#[cfg(feature = "guest")]
pub mod guest {
    pub use bevy_mod_ffi_guest::prelude;

    pub use bevy_mod_ffi_macros::main;
}

#[cfg(feature = "host")]
pub mod host {
    pub use bevy_mod_ffi_host::*;
}

#[cfg(feature = "macros")]
pub use bevy_mod_ffi_macros::*;
