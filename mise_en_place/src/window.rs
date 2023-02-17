use std::collections::HashMap;

use bevy_ecs::prelude::{Events, Resource};
use winit::event::{AxisId, ElementState, MouseButton, Touch};

use crate::{Area, Attach, BackendStages, Engen, FrontEndStages, Position};
use crate::coord::Device;
use crate::window::Orientation::{Landscape, Portrait};

#[derive(Resource, Copy, Clone)]
pub struct TouchAdapter {
    pub current_touch: Option<Touch>,
    pub end_touch: Option<Touch>,
}

impl TouchAdapter {
    pub fn new() -> Self {
        Self {
            current_touch: None,
            end_touch: None,
        }
    }
}

#[derive(Resource)]
pub struct MotionAdapter {
    pub mapping: HashMap<AxisId, f64>,
}

impl MotionAdapter {
    pub fn new() -> Self {
        Self {
            mapping: HashMap::new(),
        }
    }
}

#[derive(Resource)]
pub struct MouseAdapter {
    pub location: Option<Position<Device>>,
    pub button_state: HashMap<MouseButton, ElementState>,
}

impl MouseAdapter {
    pub fn new() -> Self {
        Self {
            location: None,
            button_state: HashMap::new(),
        }
    }
}

#[derive(Resource, Copy, Clone)]
pub enum Orientation {
    Portrait(f32),
    Landscape(f32),
}

impl Orientation {
    pub fn new<A: Into<Area<Device>>>(window_size: A) -> Self {
        let window_size = window_size.into();
        let aspect_ratio = window_size.width / window_size.height;
        match aspect_ratio >= 1.0 {
            true => Landscape(aspect_ratio),
            false => Portrait(aspect_ratio),
        }
    }
}

#[derive(Resource, Clone, Copy)]
pub struct ScaleFactor {
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
    pub(crate) size: Area<Device>,
    pub(crate) scale_factor: f64,
}

impl Resize {
    pub(crate) fn new(size: Area<Device>, scale_factor: f64) -> Self {
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
        engen
            .frontend
            .container
            .insert_resource(TouchAdapter::new());
        engen
            .frontend
            .container
            .insert_resource(MouseAdapter::new());
        engen.frontend.container.insert_resource(MotionAdapter::new());
    }
}
