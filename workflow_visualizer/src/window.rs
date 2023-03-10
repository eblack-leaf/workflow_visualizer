use bevy_ecs::prelude::Events;
use crate::area::Area;
use crate::{Attach, Engen};
use crate::coord::DeviceContext;

#[derive(Clone, Copy)]
pub struct WindowResize {
    pub size: Area<DeviceContext>,
    pub scale_factor: f64,
}

impl WindowResize {
    pub(crate) fn new(size: Area<DeviceContext>, scale_factor: f64) -> Self {
        Self { size, scale_factor }
    }
}
pub struct WindowAttachment;
impl Attach for WindowAttachment {
    fn attach(engen: &mut Engen) {
        engen
            .frontend
            .container
            .insert_resource(Events::<WindowResize>::default());
        engen
            .backend
            .container
            .insert_resource(Events::<WindowResize>::default());
        engen
            .frontend
            .main
            .add_system(Events::<WindowResize>::update_system);
        engen
            .backend
            .main
            .add_system(Events::<WindowResize>::update_system);
    }
}