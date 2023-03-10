use bevy_ecs::prelude::IntoSystemConfig;
use crate::{Attach, Engen};
use crate::time::{system, Timer};

pub struct TimerAttachment;

impl Attach for TimerAttachment {
    fn attach(engen: &mut Engen) {
        engen.frontend.container.insert_resource(Timer::new());
        engen
            .frontend
            .main
            .add_system(system::read_time);
        engen
            .frontend
            .startup
            .add_system(system::start_time);
    }
}
