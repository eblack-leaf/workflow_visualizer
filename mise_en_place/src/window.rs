use std::rc::Rc;

use bevy_ecs::prelude::{Events, Resource};
use winit::event_loop::EventLoopWindowTarget;
use winit::window::Window;

use crate::coord::ScaledArea;
use crate::{Attach, BackendStages, FrontEndStages, Stove};

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
    pub(crate) size: ScaledArea,
    pub(crate) scale_factor: f64,
}

impl Resize {
    pub(crate) fn new(size: ScaledArea, scale_factor: f64) -> Self {
        Self { size, scale_factor }
    }
}

impl Attach for Resize {
    fn attach(stove: &mut Stove) {
        stove
            .frontend
            .container
            .insert_resource(Events::<Resize>::default());
        stove
            .backend
            .container
            .insert_resource(Events::<Resize>::default());
        stove
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::First, Events::<Resize>::update_system);
        stove
            .backend
            .main
            .add_system_to_stage(BackendStages::Initialize, Events::<Resize>::update_system);
    }
}
