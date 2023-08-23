use bevy_ecs::change_detection::ResMut;
use bevy_ecs::event::EventReader;
use bevy_ecs::prelude::{Commands, Component, IntoSystemConfigs, Res, Resource};

use crate::viewport::ViewportHandle;
use crate::visualizer::{Attach, Visualizer};
use crate::window::WindowResize;
use crate::{Area, DeviceContext, NumericalContext, ScaleFactor, SyncPoint};

#[derive(Component, Resource, Copy, Clone)]
pub enum Orientation {
    Portrait(AspectRatio),
    Landscape(AspectRatio),
}

impl Orientation {
    pub fn value(&self) -> AspectRatio {
        match &self {
            Orientation::Portrait(aspect) => *aspect,
            Orientation::Landscape(aspect) => *aspect,
        }
    }
    pub fn new<A: Into<Area<NumericalContext>>>(dimensions: A) -> Self {
        let aspect_ratio = AspectRatio::new(dimensions.into());
        if let true = aspect_ratio.is_width_major() {
            Orientation::Landscape(aspect_ratio.into())
        } else {
            Orientation::Portrait(aspect_ratio.into())
        }
    }
}

pub(crate) fn setup_orientation(
    mut cmd: Commands,
    viewport_handle: Res<ViewportHandle>,
    scale_factor: Res<ScaleFactor>,
) {
    cmd.insert_resource(Orientation::new(
        viewport_handle
            .section
            .area
            .to_device(scale_factor.factor())
            .as_numerical(),
    ));
}

pub(crate) fn calc_orientation(
    mut events: EventReader<WindowResize>,
    mut orientation: ResMut<Orientation>,
) {
    for event in events.iter() {
        *orientation = Orientation::new(event.size.as_numerical());
    }
}
pub struct OrientationAttachment;
impl Attach for OrientationAttachment {
    fn attach(engen: &mut Visualizer) {
        engen
            .job
            .task(Visualizer::TASK_STARTUP)
            .add_systems((setup_orientation,));
        engen
            .job
            .task(Visualizer::TASK_MAIN)
            .add_systems((calc_orientation.in_set(SyncPoint::Config),));
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub struct AspectRatio(pub f32);

impl AspectRatio {
    pub fn new(area: Area<NumericalContext>) -> Self {
        Self(area.width / area.height)
    }
    pub fn is_width_major(&self) -> bool {
        self.0 >= 1f32
    }
    pub fn reciprocal(&self) -> f32 {
        1f32 / self.0
    }
}
impl From<f32> for AspectRatio {
    fn from(value: f32) -> Self {
        AspectRatio(value)
    }
}
