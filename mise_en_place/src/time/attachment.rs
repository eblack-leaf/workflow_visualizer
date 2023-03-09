use bevy_ecs::prelude::IntoSystemConfig;

use crate::time::system;
use crate::{Attach, Engen, FrontEndBuckets, FrontEndStartupBuckets, Timer};

pub struct TimerAttachment;

impl Attach for TimerAttachment {
    fn attach(engen: &mut Engen) {
        engen.frontend.container.insert_resource(Timer::new());
        engen
            .frontend
            .main
            .add_system(system::read_time.in_set(FrontEndBuckets::First));
        engen
            .frontend
            .startup
            .add_system(system::start_time.in_set(FrontEndStartupBuckets::Last));
    }
}
