use std::collections::{HashMap, HashSet};

use bevy_ecs::prelude::{Events, ResMut, Resource};
use winit::event::{AxisId, ElementState, MouseButton};

use crate::coord::Device;
use crate::window::Orientation::{Landscape, Portrait};
use crate::{Area, Attach, BackendStages, Engen, FrontEndStages, Position};

#[derive(Copy, Clone)]
pub struct Click {
    pub origin: Position<Device>,
    pub current: Option<Position<Device>>,
    pub end: Option<Position<Device>>,
}

impl Click {
    pub fn new<PD: Into<Position<Device>>>(origin: PD) -> Self {
        let position = origin.into();
        Self {
            origin: position,
            current: Some(position),
            end: None,
        }
    }
}

pub type Finger = u32;

#[derive(Resource)]
pub struct TouchAdapter {
    pub primary: Option<Finger>,
    pub tracked: HashMap<Finger, Click>,
    pub primary_end_event: Option<(Finger, Click)>,
}

impl TouchAdapter {
    pub fn new() -> Self {
        Self {
            primary: None,
            tracked: HashMap::new(),
            primary_end_event: None,
        }
    }
}

#[derive(Resource)]
pub struct MouseAdapter {
    pub location: Option<Position<Device>>,
    pub tracked_buttons: HashMap<MouseButton, Click>,
    pub valid_releases: HashMap<MouseButton, Click>,
}

impl MouseAdapter {
    pub fn new() -> Self {
        Self {
            location: None,
            tracked_buttons: HashMap::new(),
            valid_releases: HashMap::new(),
        }
    }
}

pub(crate) fn reset_adapters(
    mut touch_adapter: ResMut<TouchAdapter>,
    mut mouse_adapter: ResMut<MouseAdapter>,
) {
    touch_adapter.primary_end_event.take();
    mouse_adapter.valid_releases.clear();
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

pub struct WindowPlugin;

impl Attach for WindowPlugin {
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
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::Last, reset_adapters);
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
    }
}
