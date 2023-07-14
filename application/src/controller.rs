use workflow_visualizer::{Attach, UserSpaceSyncPoint, Visualizer};
use workflow_visualizer::bevy_ecs;
use workflow_visualizer::bevy_ecs::prelude::{Entity, IntoSystemConfig, Resource};

use crate::system;

pub(crate) struct Slot {
    pub(crate) name_text: Entity,
    pub(crate) otp_text: Entity,
    pub(crate) generate_button: Entity,
    pub(crate) delete_button: Entity,
}

#[derive(Resource)]
pub(crate) struct SlotController {
    pub(crate) slots: Vec<Slot>,
}

impl Attach for SlotController {
    fn attach(visualizer: &mut Visualizer) {
        visualizer
            .job
            .container
            .insert_resource(SlotController { slots: vec![] });
        visualizer
            .job
            .task(Visualizer::TASK_STARTUP)
            .add_systems((system::setup.in_set(UserSpaceSyncPoint::Initialization), ));
        visualizer
            .job
            .task(Visualizer::TASK_MAIN)
            .add_systems((system::send_event.in_set(UserSpaceSyncPoint::Process), ));
    }
}
