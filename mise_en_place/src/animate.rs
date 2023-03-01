use bevy_ecs::prelude::{Added, Commands, Component, Entity, ParamSet, Query, Res};

use crate::{Attach, Engen, FrontEndStages, Position, PositionAdjust, UIView};
use crate::time::{TimeDelta, TimeMarker, Timer};

#[derive(Component)]
pub struct Animation<T: Component> {
    pub total_time: TimeDelta,
    pub start: Option<TimeMarker>,
    pub animator: T,
}

impl<T: Component> Animation<T> {
    pub fn new(total_time: TimeDelta, animator: T) -> Self {
        Self {
            total_time,
            start: None,
            animator,
        }
    }
    pub fn calc_delta_factor(&mut self, timer: &Timer) -> (f32, bool) {
        let total = timer.time_since(self.start.unwrap());
        let done = total >= self.total_time;
        let past_total = total - self.total_time;
        let mut delta = timer.frame_diff();
        if done {
            delta -= past_total;
        }
        let delta = delta / self.total_time;
        (delta.as_f32(), done)
    }
}

pub trait Animate {
    type Animator: Component;
    fn animate<T: Into<TimeDelta>>(self, total_time: T) -> Animation<Self::Animator>;
}

pub fn start_animations<T: Component>(
    mut uninitialized_animations: Query<&mut Animation<T>, Added<Animation<T>>>,
    timer: Res<Timer>,
) {
    for mut anim in uninitialized_animations.iter_mut() {
        anim.start.replace(timer.mark());
    }
}
