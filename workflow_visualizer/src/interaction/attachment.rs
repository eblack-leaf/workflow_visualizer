use bevy_ecs::prelude::IntoSystemConfigs;

pub(crate) use crate::interaction::interaction::resolve;
use crate::interaction::interaction::{
    cleanup, update_interactions, InteractionEvent, InteractionLocations, InteractionPhases,
    MouseAdapter, PrimaryInteraction, PrimaryMouseButton,
};
use crate::{Attach, SyncPoint, Visualizer};

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
