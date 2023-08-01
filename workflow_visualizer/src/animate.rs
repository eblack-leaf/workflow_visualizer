use bevy_ecs::prelude::{Changed, Commands, Component, Entity, Query, Res};

use crate::{TimeDelta, TimeMarker, Timer};

/// Animation class for running interpolations over time
#[derive(Component, Clone)]
pub struct Animation<T: Clone> {
    pub total_time: TimeDelta,
    pub animation_time: TimeDelta,
    pub start: Option<TimeMarker>,
    pub animator: T,
    done: bool,
    delay: Option<TimeDelta>,
}

impl<T: Clone> Animation<T> {
    pub fn new<TD: Into<TimeDelta> + Copy + Clone>(
        animator: T,
        total_time: TD,
        delay_time: Option<TD>,
    ) -> Self {
        let delay = if let Some(time) = delay_time {
            Some(time.into())
        } else {
            None
        };
        Self {
            total_time: total_time.into() + delay.unwrap_or_default(),
            animation_time: total_time.into(),
            start: None,
            animator,
            done: false,
            delay,
        }
    }
    pub fn calc_delta_factor(&mut self, timer: &Timer) -> (f32, bool) {
        if let Some(start) = self.start {
            return if !self.delayed(timer) {
                let mut time_since_start = timer.time_since(start);
                let done = time_since_start >= self.total_time;
                let mut delta = timer.frame_diff();
                if done {
                    let past_total = time_since_start - self.total_time;
                    delta -= past_total;
                    self.done = true;
                }
                let delta = delta / self.animation_time;
                (delta.as_f32(), done)
            } else {
                (0f32, false)
            }
        } else {
            (0f32, false)
        }
    }
    pub fn done(&self) -> bool {
        self.done
    }
    pub fn delayed(&self, timer: &Timer) -> bool {
        if let Some(delay) = self.delay {
            return if let Some(start) = self.start {
                if timer.time_since(start) < delay {
                    true
                } else {
                    false
                }
            } else {
                false
            };
        }
        false
    }
}
/// starter for the animations. Must be added for animation to run
#[allow(unused)]
pub fn start_animations<T: Send + Sync + 'static + Clone>(
    mut uninitialized_animations: Query<&mut Animation<T>, Changed<Animation<T>>>,
    timer: Res<Timer>,
) {
    for mut anim in uninitialized_animations.iter_mut() {
        let mark = timer.mark();
        if anim.start.is_none() {
            anim.start.replace(mark);
        }
    }
}

pub fn end_animations<T: Send + Sync + 'static + Clone>(
    mut ended: Query<(Entity, &Animation<T>)>,
    timer: Res<Timer>,
    mut cmd: Commands,
) {
    for (entity, anim) in ended.iter() {
        if anim.done() {
            cmd.entity(entity).remove::<Animation<T>>();
            // send done signal
        }
    }
}
/// Interpolates a value over an interval
#[derive(Copy, Clone)]
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
