use crate::{TimeDelta, TimeMarker, Timer};
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::Bundle;
use bevy_ecs::system::{Commands, Query, Res};

#[derive(Bundle)]
pub struct BundleBuilder<T: Bundle, S: Bundle> {
    pub original: T,
    pub extension: S,
}

impl<T: Bundle, S: Bundle> BundleBuilder<T, S> {
    pub fn new(t: T, s: S) -> Self {
        Self {
            original: t,
            extension: s,
        }
    }
}

pub trait BundleExtension
where
    Self: Bundle + Sized,
{
    fn extend<E: Bundle>(self, handle: E) -> BundleBuilder<Self, E>;
}

impl<I: Bundle> BundleExtension for I {
    fn extend<E: Bundle>(self, handle: E) -> BundleBuilder<I, E> {
        BundleBuilder::new(self, handle)
    }
}
#[derive(Component)]
pub struct DelayedBundle<T: Bundle + Sized + Clone> {
    pub bundle: T,
    pub start: Option<TimeMarker>,
    pub delay: TimeDelta,
}
impl<T: Bundle + Sized + Clone> DelayedBundle<T> {
    pub fn new<TD: Into<TimeDelta>>(bundle: T, delay: TD) -> Self {
        Self {
            bundle,
            start: None,
            delay: delay.into(),
        }
    }
}
pub trait DelayedSpawn
where
    Self: Bundle + Sized + Clone,
{
    fn delay<TD: Into<TimeDelta>>(self, delay: TD) -> DelayedBundle<Self>;
}
impl<T: Bundle + Sized + Clone> DelayedSpawn for T {
    fn delay<TD: Into<TimeDelta>>(self, delay: TD) -> DelayedBundle<Self> {
        DelayedBundle::<T>::new(self, delay)
    }
}
pub(crate) fn spawn_delayed_bundle<T: Bundle + Sized + Send + 'static + Clone>(
    mut delayed: Query<(Entity, &mut DelayedBundle<T>)>,
    timer: Res<Timer>,
    mut cmd: Commands,
) {
    for (entity, mut delayed_bundle) in delayed.iter_mut() {
        if delayed_bundle.start.is_none() {
            delayed_bundle.start.replace(timer.mark());
        }
        let time_since_start = timer.time_since(delayed_bundle.start.unwrap());
        if time_since_start >= delayed_bundle.delay {
            cmd.entity(entity).insert(delayed_bundle.bundle.clone());
            cmd.entity(entity).remove::<DelayedBundle<T>>();
        }
    }
}
