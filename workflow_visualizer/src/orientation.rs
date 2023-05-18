use bevy_ecs::change_detection::ResMut;
use bevy_ecs::event::EventReader;
use bevy_ecs::prelude::{Commands, IntoSystemConfig, Res, Resource};

use crate::viewport::ViewportHandle;
use crate::window::WindowResize;
use crate::{Area, Attach, DeviceContext, ScaleFactor, SyncPoint, Visualizer};

#[derive(Resource, Copy, Clone)]
pub enum Orientation {
    Portrait(f32),
    Landscape(f32),
}

impl Orientation {
    pub fn new<A: Into<Area<DeviceContext>>>(window_size: A) -> Self {
        let window_size = window_size.into();
        let aspect_ratio = window_size.width / window_size.height;
        match aspect_ratio >= 1.0 {
            true => Orientation::Landscape(aspect_ratio),
            false => Orientation::Portrait(aspect_ratio),
        }
    }
}

pub(crate) fn setup_orientation(
    mut cmd: Commands,
    viewport_handle: Res<ViewportHandle>,
    scale_factor: Res<ScaleFactor>,
) {
    cmd.insert_resource(Orientation::new(
        viewport_handle.section.area.to_device(scale_factor.factor),
    ));
}

pub(crate) fn calc_orientation(
    mut events: EventReader<WindowResize>,
    mut orientation: ResMut<Orientation>,
) {
    for event in events.iter() {
        *orientation = Orientation::new(event.size);
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
