use bevy::ecs::system::DynParamBuilder;
use bevy::prelude::*;

pub struct ParamBuilderAccumulator {
    pub builders: Vec<DynParamBuilder<'static>>,
}
