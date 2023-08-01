use std::collections::HashMap;

use bevy_ecs::prelude::{Entity, Res, ResMut, Resource};

use crate::{TimeDelta, TimeMarker, Timer};

#[derive(Resource)]
pub struct AnimationManager<T> {
    pub managed_animations: HashMap<Entity, ManagedAnimation<T>>,
}
impl<T> AnimationManager<T> {
    pub(crate) fn new() -> Self {
        Self {
            managed_animations: HashMap::new(),
        }
    }
    pub fn animate<TD: Into<TimeDelta>>(
        &mut self,
        entity: Entity,
        animator: T,
        anim_time: TD,
        delay: Option<TD>,
    ) {
        if let Some(managed_animation) = self.managed_animations.get_mut(&entity) {
            if let Some(delay) = delay {
                managed_animation.delay(animator, anim_time, delay);
            } else {
                managed_animation.animate(animator, anim_time);
            }
        } else {
            if let Some(delay) = delay {
                let mut anim = ManagedAnimation::new();
                anim.delay(animator, anim_time, delay);
                self.managed_animations.insert(entity, anim);
            } else {
                let mut anim = ManagedAnimation::new();
                anim.animate(animator, anim_time);
                self.managed_animations.insert(entity, anim);
            }
        }
    }
}

pub(crate) fn manage<T: Send + Sync + 'static>(
    mut animation_manager: ResMut<AnimationManager<T>>,
    timer: Res<Timer>,
) {
    for (entity, managed_animation) in animation_manager.managed_animations.iter_mut() {
        let mut done_delaying = vec![];
        let mut index = 0;
        for (delay, anim) in managed_animation.queue.iter_mut() {
            *delay -= timer.frame_diff();
            if *delay <= 0.into() {
                done_delaying.push(index);
            }
            index += 1;
        }
        if done_delaying.is_empty() {
            if let Some(current) = managed_animation.current.as_mut() {
                if current.start.is_none() {
                    current.start.replace(timer.mark());
                }
            }
        } else {
            done_delaying.sort();
            done_delaying.reverse();
            for index in done_delaying.drain(..) {
                let (overage, mut anim) = managed_animation.queue.remove(index);
                anim.start.replace(timer.mark().offset(overage));
                managed_animation.current.replace(anim);
            }
        }
        if let Some(current) = managed_animation.current.as_mut() {
            if let Some(start) = current.start {
                let mut time_since_start = timer.time_since(start);
                let done = time_since_start >= current.animation_time;
                let mut anim_delta: TimeDelta = timer.frame_diff().0.min(time_since_start.0).into();
                if done {
                    let past_total = time_since_start - current.animation_time;
                    anim_delta -= past_total;
                    current.done = true;
                }
                let anim_delta = anim_delta / current.animation_time;
                current.delta.replace(anim_delta.0.min(1f64) as f32);
            }
        }
    }
}

pub(crate) fn end_animations<T: Send + Sync + 'static>(
    mut animation_manager: ResMut<AnimationManager<T>>,
) {
    let mut removals = vec![];
    for (entity, managed_anim) in animation_manager.managed_animations.iter() {
        if let Some(current) = managed_anim.current.as_ref() {
            if current.done {
                removals.push(*entity);
            }
        }
    }
    for entity in removals {
        let _ = animation_manager
            .managed_animations
            .get_mut(&entity)
            .expect("anim")
            .current
            .take();
    }
}
pub struct ManagedAnimation<T> {
    pub current: Option<Animation<T>>,
    pub(crate) queue: Vec<(TimeDelta, Animation<T>)>,
}
impl<T> ManagedAnimation<T> {
    pub(crate) fn new() -> Self {
        Self {
            current: None,
            queue: vec![],
        }
    }
    pub(crate) fn animate<TD: Into<TimeDelta>>(&mut self, animator: T, anim_time: TD) {
        self.current
            .replace(Animation::new(anim_time.into(), animator));
    }
    pub(crate) fn delay<TD: Into<TimeDelta>>(&mut self, animator: T, anim_time: TD, delay: TD) {
        self.queue
            .push((delay.into(), Animation::new(anim_time.into(), animator)));
    }
}
#[derive(Clone)]
pub struct Animation<T> {
    pub(crate) start: Option<TimeMarker>,
    pub(crate) animation_time: TimeDelta,
    pub animator: T,
    pub(crate) delta: Option<f32>,
    pub(crate) done: bool,
}

impl<T> Animation<T> {
    pub(crate) fn new<TD: Into<TimeDelta>>(anim_time: TD, animator: T) -> Self {
        Self {
            start: None,
            animation_time: anim_time.into(),
            animator,
            delta: None,
            done: false,
        }
    }
    pub fn done(&self) -> bool {
        self.done
    }
    pub fn set_done(&mut self) {
        self.done = true;
    }
    pub fn delta(&mut self) -> Option<f32> {
        if let Some(delta) = self.delta.take() {
            return if self.done() { Some(1f32) } else { Some(delta) };
        }
        None
    }
}
#[derive(Clone, Copy)]
pub struct InterpolationExtraction(pub f32, pub bool);

impl InterpolationExtraction {}
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
    pub fn extract(&mut self, delta: f32) -> InterpolationExtraction {
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
        let extract = InterpolationExtraction(extract, done);
        extract
    }
}

#[cfg(test)]
#[test]
pub(crate) fn interpolator_test() {
    let mut interpolator = Interpolator::new(1f32);
    let extraction = interpolator.extract(0.25);
    assert_eq!(extraction.0, 0.25f32);
}
