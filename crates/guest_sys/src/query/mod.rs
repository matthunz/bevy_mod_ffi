use bevy_mod_ffi_core::*;

pub mod builder;
pub mod iter;
pub mod state;

pub use builder::*;
pub use iter::*;
pub use state::*;

unsafe extern "C" {
    pub fn bevy_query_iter_mut(query: *mut query, out_iter: *mut *mut query_iter) -> bool;

    pub fn bevy_query_drop(iter: *mut query);
}
