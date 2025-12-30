use bevy::ecs::system::{DynSystemParam, SystemState};

pub mod param;

pub type SharedSystemState = SystemState<(Vec<DynSystemParam<'static, 'static>>,)>;

pub use crate::GuestRunSystemFnType as GuestRunSystemFn;
