use std::sync::Arc;

use crate::{Attach, BackendStages, Engen, FrontEndStages};
use bevy_ecs::prelude::{Events, Resource};
use winit::event_loop::EventLoopWindowTarget;
use winit::window::Window;

use crate::coord::ScaledArea;

#[derive(Resource)]
pub(crate) struct EngenWindow {
    pub(crate) window_ref: Arc<Window>,
}

impl EngenWindow {
    pub(crate) fn new(event_loop_window_target: &EventLoopWindowTarget<()>) -> Self {
        Self {
            window_ref: Arc::new(Window::new(event_loop_window_target).expect("window new")),
        }
    }
}

#[derive(Resource)]
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
            .add_system_to_stage(FrontEndStages::Initialize, Events::<Resize>::update_system);
        engen
            .backend
            .main
            .add_system_to_stage(BackendStages::Initialize, Events::<Resize>::update_system);
    }
}
