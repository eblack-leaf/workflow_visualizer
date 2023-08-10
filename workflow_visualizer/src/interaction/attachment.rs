use bevy_ecs::prelude::IntoSystemConfig;

use crate::{Attach, SyncPoint, Visualizer};
use crate::interaction::interaction::{
    cleanup, InteractionEvent, InteractionLocations, InteractionPhases, MouseAdapter,
    PrimaryInteraction, PrimaryMouseButton, update_interactions,
};
pub(crate) use crate::interaction::interaction::resolve;

pub(crate) struct InteractionAttachment;

impl Attach for InteractionAttachment {
    fn attach(visualizer: &mut Visualizer) {
        visualizer.add_event::<InteractionEvent>();
        visualizer
            .job
            .container
            .insert_resource(PrimaryMouseButton::default());
        visualizer
            .job
            .container
            .insert_resource(PrimaryInteraction::default());
        visualizer
            .job
            .container
            .insert_resource(InteractionPhases::default());
        visualizer
            .job
            .container
            .insert_resource(InteractionLocations::default());
        visualizer
            .job
            .container
            .insert_resource(MouseAdapter::default());
        visualizer.job.task(Visualizer::TASK_MAIN).add_systems((
            update_interactions.in_set(SyncPoint::PostInitialization),
            resolve.in_set(SyncPoint::Preparation),
            cleanup.in_set(SyncPoint::Finish),
        ));
    }
}
