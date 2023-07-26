use bevy_ecs::event::Events;
use bevy_ecs::prelude::IntoSystemConfig;

use crate::{Attach, PrimaryTouch, SyncPoint, Visualizer};
use crate::touch::adapter::{MouseAdapter, TouchAdapter, TouchGrabState};
use crate::touch::component::TouchEvent;
use crate::touch::system;

pub(crate) struct TouchAttachment;

impl Attach for TouchAttachment {
    fn attach(visualizer: &mut Visualizer) {
        visualizer
            .job
            .container
            .insert_resource(PrimaryTouch::new());
        visualizer
            .job
            .container
            .insert_resource(TouchGrabState::new());
        visualizer
            .job
            .container
            .insert_resource(Events::<TouchEvent>::default());
        visualizer
            .job
            .container
            .insert_resource(TouchAdapter::new());
        visualizer
            .job
            .container
            .insert_resource(MouseAdapter::new());
        visualizer.job.task(Visualizer::TASK_MAIN).add_systems((
            Events::<TouchEvent>::update_system.in_set(SyncPoint::Event),
            system::read_touch_events.in_set(SyncPoint::Preparation),
            system::reset_touched.in_set(SyncPoint::Finish),
        ));
    }
}
