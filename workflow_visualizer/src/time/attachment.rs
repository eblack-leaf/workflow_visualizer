use bevy_ecs::prelude::IntoSystemConfigs;

use crate::time::{system, TimeTracker};
use crate::visualizer::{Attach, Visualizer};
use crate::SyncPoint;

pub struct TimerAttachment;

impl Attach for TimerAttachment {
    fn attach(engen: &mut Visualizer) {
        engen.job.container.insert_resource(TimeTracker::new());
        engen
            .job
            .task(Visualizer::TASK_MAIN)
            .add_systems((system::read_time.in_set(SyncPoint::Event),));
        engen
            .job
            .task(Visualizer::TASK_STARTUP)
            .add_systems((system::start_time,));
    }
}
