use bevy_ecs::prelude::{Events, Resource};

use crate::{Area, Attach, BackendStages, Engen, FrontEndStages};
use crate::coord::Scaled;

#[derive(Resource, Clone, Copy)]
pub(crate) struct ScaleFactor {
    pub(crate) factor: f64,
}

impl ScaleFactor {
    pub(crate) fn new(factor: f64) -> Self {
        Self { factor }
    }
}

impl From<f64> for ScaleFactor {
    fn from(value: f64) -> Self {
        Self::new(value)
    }
}

#[derive(Clone, Copy)]
pub(crate) struct Resize {
    pub(crate) size: Area<Scaled>,
    pub(crate) scale_factor: f64,
}

impl Resize {
    pub(crate) fn new(size: Area<Scaled>, scale_factor: f64) -> Self {
        Self { size, scale_factor }
    }
}

impl Attach for Resize {
    fn attach(engen: &mut Engen) {
        engen
            .frontend
            .container
            .insert_resource(Events::<Resize>::default());
        engen
            .backend
            .container
            .insert_resource(Events::<Resize>::default());
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::First, Events::<Resize>::update_system);
        engen
            .backend
            .main
            .add_system_to_stage(BackendStages::Initialize, Events::<Resize>::update_system);
    }
}
