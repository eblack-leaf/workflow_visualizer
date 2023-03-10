use crate::time::Timer;
use bevy_ecs::prelude::ResMut;

pub(crate) fn read_time(mut timer: ResMut<Timer>) {
    let _delta = timer.read();
}

pub(crate) fn start_time(mut timer: ResMut<Timer>) {
    timer.set_to_now();
}
