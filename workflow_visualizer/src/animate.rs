use std::marker::PhantomData;

use bevy_ecs::prelude::{Changed, Component, Entity, Query, Res};
use bevy_ecs::system::Commands;

use crate::{TimeDelta, TimeTracker, Timer};
#[derive(Copy, Clone, Default, Debug, Component)]
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

#[derive(Clone, Copy, Default, Debug)]
pub struct InterpolationExtraction(pub f32);
impl InterpolationExtraction {
    pub fn value(&self) -> f32 {
        self.0
    }
}

impl From<f32> for InterpolationExtraction {
    fn from(value: f32) -> Self {
        InterpolationExtraction(value)
    }
}

#[derive(Component)]
pub struct Animation<T: Animate> {
    timer: Timer,
    interpolations: Vec<Interpolation>,
    done: bool,
    end: Option<T>,
    _phantom: PhantomData<T>,
}

impl<T: Animate> Animation<T> {
    pub fn new(timer: Timer, interpolations: Vec<Interpolation>, end: T) -> Self {
        Self {
            timer,
            interpolations,
            done: false,
            end: Some(end),
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
    pub fn with_offset<TD: Into<TimeDelta>>(mut self, offset: Option<TD>) -> Self {
        self.timer.set_offset(offset);
        self
    }
}

pub trait Animate
where
    Self: Sized + Clone,
{
    fn interpolations(&self, end: &Self) -> Vec<Interpolation>;
    fn animate<TD: Into<TimeDelta>>(&self, end: Self, animation_time: TD) -> QueuedAnimation<Self> {
        let timer = Timer::new(animation_time);
        QueuedAnimation(Some(Animation::new(
            timer,
            Self::interpolations(self, &end),
            end.clone(),
        )))
    }
}
#[derive(Component, Default)]
pub struct QueuedAnimation<T: Animate>(pub Option<Animation<T>>);
impl<T: Animate> QueuedAnimation<T> {
    pub fn with_offset<TD: Into<TimeDelta>>(mut self, offset: Option<TD>) -> Self {
        self.0.expect("queued anim").timer.set_offset(offset);
        self
    }
}
pub(crate) fn pull_from_queue<T: Animate + Send + Sync + 'static + Component>(
    mut queued: Query<(Entity, &mut QueuedAnimation<T>, Option<&mut Animation<T>>)>,
    mut cmd: Commands,
) {
    for (entity, mut queued, mut current) in queued.iter_mut() {
        if let Some(current) = current.as_mut() {
            cmd.entity(entity)
                .insert(current.end.take().expect("no anim end"));
        }
        cmd.entity(entity)
            .insert(queued.0.take().expect("no queued anim"));
    }
}
pub(crate) fn start_animations<T: Animate + Send + Sync + 'static>(
    mut animations: Query<&mut Animation<T>, Changed<Animation<T>>>,
    time_tracker: Res<TimeTracker>,
) {
    for mut animation in animations.iter_mut() {
        if animation.timer.not_started() {
            animation.timer.start(time_tracker.mark());
        }
    }
}

pub(crate) fn update_animations<T: Animate + Send + Sync + 'static>(
    mut animations: Query<&mut Animation<T>>,
    time_tracker: Res<TimeTracker>,
) {
    for mut animation in animations.iter_mut() {
        if animation.timer.started() {
            animation.timer.mark(time_tracker.mark());
            if let Some(_elapsed) = animation.timer.time_elapsed() {
                let percent = animation.timer.percent_elapsed(time_tracker.frame_diff());
                if animation.timer.finished() {
                    animation.done = true;
                }
                let anim_done = animation.done;
                let mut all_finished = true;
                for interpolation in animation.interpolations.iter_mut() {
                    if anim_done {
                        let _extract = interpolation.finish();
                    } else {
                        let _extract = interpolation.extract(percent);
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
