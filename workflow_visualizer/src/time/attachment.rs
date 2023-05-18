use bevy_ecs::prelude::IntoSystemConfig;

use crate::time::{system, Timer};
use crate::SyncPoint;
use crate::{Attach, Visualizer};

pub struct TimerAttachment;

impl Attach for TimerAttachment {
    fn attach(engen: &mut Visualizer) {
        engen.job.container.insert_resource(Timer::new());
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
