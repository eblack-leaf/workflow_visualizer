use std::marker::PhantomData;

use bevy_ecs::prelude::{Changed, Component, Entity, Query, Res};
use bevy_ecs::system::Commands;

use crate::{TimeDelta, TimeMarker, Timer};

pub struct Interpolation {
    remaining: f32,
    total: f32,
    extraction: Option<InterpolationExtraction>,
}

impl Interpolation {
    pub fn new(total: f32) -> Self {
        Self {
            remaining: total,
            total,
            extraction: None,
        }
    }
    pub fn done(&self) -> bool {
        self.remaining == 0f32
    }
    pub fn extract(&mut self, percent: f32) -> Option<InterpolationExtraction> {
        if !self.done() {
            let delta = self.total * percent;
            if delta.abs() > self.remaining.abs() {
                return self.finish();
            }
            self.remaining -= delta;
            self.extraction.replace(delta.into());
            return Some(delta.into());
        }
        self.extraction.take();
        None
    }
    pub fn finish(&mut self) -> Option<InterpolationExtraction> {
        if !self.done() {
            let remaining = self.remaining.into();
            self.extraction.replace(remaining);
            self.remaining = 0f32;
            return Some(remaining);
        }
        self.extraction.take();
        None
    }
}

#[derive(Clone, Copy)]
pub struct InterpolationExtraction(pub f32);

impl From<f32> for InterpolationExtraction {
    fn from(value: f32) -> Self {
        InterpolationExtraction(value)
    }
}

#[derive(Component)]
pub struct Animation<T: Animate> {
    start: Option<TimeMarker>,
    start_offset: Option<TimeDelta>,
    animation_time: TimeDelta,
    interpolations: Vec<Interpolation>,
    done: bool,
    _phantom: PhantomData<T>,
}

impl<T: Animate> Animation<T> {
    pub fn new(
        animation_time: TimeDelta,
        start_offset: Option<TimeDelta>,
        interpolations: Vec<Interpolation>,
    ) -> Self {
        Self {
            start: None,
            start_offset,
            animation_time,
            interpolations,
            done: false,
            _phantom: PhantomData,
        }
    }
    pub fn extractions(&mut self) -> Vec<Option<InterpolationExtraction>> {
        let mut extractions = vec![];
        for interpolation in self.interpolations.iter_mut() {
            extractions.push(interpolation.extraction.take());
        }
        extractions
    }
    pub fn done(&self) -> bool {
        self.done
    }
}

pub trait Animate
where
    Self: Sized,
{
    fn interpolations(&self, end: Self) -> Vec<Interpolation>;
    fn animate(
        &self,
        end: Self,
        animation_time: TimeDelta,
        start_offset: Option<TimeDelta>,
    ) -> Animation<Self> {
        Animation::new(
            animation_time,
            start_offset,
            Self::interpolations(&self, end),
        )
    }
}

pub(crate) fn start_animations<T: Animate + Send + Sync + 'static>(
    mut animations: Query<&mut Animation<T>, Changed<Animation<T>>>,
    timer: Res<Timer>,
) {
    for mut animation in animations.iter_mut() {
        if animation.start.is_none() {
            let start = timer
                .mark()
                .offset(animation.start_offset.unwrap_or_default());
            animation.start.replace(start);
        }
    }
}

pub(crate) fn update_animations<T: Animate + Send + Sync + 'static>(
    mut animations: Query<&mut Animation<T>>,
    timer: Res<Timer>,
) {
    for mut animation in animations.iter_mut() {
        if animation.start.is_some() {
            let time_since_start = timer.time_since(animation.start.unwrap());
            if time_since_start.0.is_sign_positive() {
                let mut delta = timer.frame_diff();
                let anim_time = animation.animation_time;
                let overage = time_since_start - anim_time;
                let anim_done = overage.0.is_sign_positive() || overage.0 == 0.0;
                if anim_done {
                    delta -= overage;
                    animation.done = true;
                }
                let mut all_finished = true;
                for interpolation in animation.interpolations.iter_mut() {
                    if anim_done {
                        let _extract = interpolation.finish();
                    } else {
                        let percent = delta.0 / anim_time.0;
                        let _extract = interpolation.extract(percent as f32);
                        if !interpolation.done() {
                            all_finished = false;
                        }
                    }
                }
                if all_finished {
                    animation.done = true;
                }
            }
        }
    }
}

pub(crate) fn end_animations<T: Animate + Send + Sync + 'static>(
    animations: Query<(Entity, &Animation<T>)>,
    mut cmd: Commands,
) {
    for (entity, animation) in animations.iter() {
        if animation.done {
            cmd.entity(entity).remove::<Animation<T>>();
        }
    }
}
