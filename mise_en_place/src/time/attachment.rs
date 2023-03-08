use crate::time::system;
use crate::{Attach, Engen, FrontEndStages, FrontEndStartupStages, Timer};

pub struct TimerAttachment;

impl Attach for TimerAttachment {
    fn attach(engen: &mut Engen) {
        engen.frontend.container.insert_resource(Timer::new());
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::First, system::read_time);
        engen
            .frontend
            .startup
            .add_system_to_stage(FrontEndStartupStages::Last, system::start_time);
    }
}
