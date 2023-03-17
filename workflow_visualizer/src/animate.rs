use bevy_ecs::prelude::{Added, Component, Query, Res};

use crate::{TimeDelta, TimeMarker, Timer};

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
#[allow(unused)]
pub fn start_animations<T: Component>(
    mut uninitialized_animations: Query<&mut Animation<T>, Added<Animation<T>>>,
    timer: Res<Timer>,
) {
    for mut anim in uninitialized_animations.iter_mut() {
        let mark = timer.mark();
        anim.start.replace(mark);
    }
}
pub struct Interpolator {
    pub value: f32,
    total: f32,
    sign_positive: bool,
}

impl Interpolator {
    pub fn new(value: f32) -> Self {
        Self {
            value,
            total: value,
            sign_positive: value.is_sign_positive(),
        }
    }
    pub fn extract(&mut self, delta: f32) -> (f32, bool) {
        let segment = self.total * delta;
        self.value -= segment;
        let overage = match self.sign_positive {
            true => {
                let mut val = None;
                if self.value.is_sign_negative() {
                    val = Some(self.value)
                }
                val
            }
            false => {
                let mut val = None;
                if self.value.is_sign_positive() {
                    val = Some(self.value)
                }
                val
            }
        };
        let mut extract = segment;
        let mut done = false;
        if let Some(over) = overage {
            extract += over;
            done = true;
        }
        if extract == 0.0 {
            done = true;
        }
        (extract, done)
    }
}
