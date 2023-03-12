use crate::engen::{Attach, Engen};
use crate::time::{system, Timer};
use crate::SyncPoint;
use bevy_ecs::prelude::IntoSystemConfig;

pub struct TimerAttachment;

impl Attach for TimerAttachment {
    fn attach(engen: &mut Engen) {
        engen.frontend.container.insert_resource(Timer::new());
        engen
            .frontend
            .main
            .add_system(system::read_time.in_set(SyncPoint::Event));
        engen.frontend.startup.add_system(system::start_time);
    }
}
