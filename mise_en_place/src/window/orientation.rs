use bevy_ecs::change_detection::ResMut;
use bevy_ecs::event::EventReader;
use bevy_ecs::prelude::{Commands, Res, Resource};

use crate::window::resize::WindowResize;
use crate::window::scale_factor::ScaleFactor;
use crate::{Area, DeviceView, VisibleBounds};

#[derive(Resource, Copy, Clone)]
pub enum Orientation {
    Portrait(f32),
    Landscape(f32),
}

impl Orientation {
    pub fn new<A: Into<Area<DeviceView>>>(window_size: A) -> Self {
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
    visible_bounds: Res<VisibleBounds>,
    scale_factor: Res<ScaleFactor>,
) {
    cmd.insert_resource(Orientation::new(
        visible_bounds.section.area.to_device(scale_factor.factor),
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
