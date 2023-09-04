use bevy_ecs::prelude::ResMut;

use crate::time::TimeTracker;

pub(crate) fn read_time(mut timer: ResMut<TimeTracker>) {
    let _delta = timer.read();
}

pub(crate) fn start_time(mut timer: ResMut<TimeTracker>) {
    timer.set_to_now();
}
