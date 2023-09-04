use crate::{Interpolation, TimeDelta, TimeMarker};
use bevy_ecs::prelude::Component;

#[derive(Component, Copy, Clone, Default, Debug)]
pub struct Timer {
    pub start: Option<TimeMarker>,
    pub interval: TimeDelta,
}
impl Timer {
    pub fn new<TD: Into<TimeDelta>>(interval: TD) -> Self {
        let interval = interval.into();
        Self {
            start: None,
            interval,
        }
    }
    pub fn finished<TM: Into<TimeMarker>>(&mut self, now: TM) -> bool {
        if let Some(start) = self.start {
            let now = now.into();
            if now - start > self.interval {
                self.finish();
                return true;
            }
        }
        false
    }
    pub fn set_interval<TD: Into<TimeDelta>>(&mut self, interval: TD) {
        self.interval = interval.into();
    }
    pub fn finish(&mut self) {
        self.start.take();
    }
    pub fn start<TM: Into<TimeMarker>>(&mut self, now: TM) {
        self.start.replace(now.into());
    }
    pub fn not_started(&self) -> bool {
        self.start.is_none()
    }
}
