use bevy_ecs::prelude::ResMut;
use crate::time::Timer;


pub(crate) fn read_time(mut timer: ResMut<Timer>) {
    let _delta = timer.read();
}

pub(crate) fn start_time(mut timer: ResMut<Timer>) {
    timer.set_to_now();
}
