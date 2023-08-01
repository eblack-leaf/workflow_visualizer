use std::ops::{Add, AddAssign, Div, Sub, SubAssign};
use std::time::Instant;

use bevy_ecs::prelude::Resource;
/// Access to time on the platform
#[derive(Resource)]
pub struct Timer {
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) beginning: Instant,
    #[cfg(target_arch = "wasm32")]
    pub(crate) beginning: f64,
    pub current: f64,
    pub last: f64,
}

#[cfg(target_arch = "wasm32")]
fn millisecond_to_sec(ms: f64) -> f64 {
    ms / 1000.0
}

impl Timer {
    pub(crate) fn new() -> Self {
        Self {
            #[cfg(not(target_arch = "wasm32"))]
            beginning: Instant::now(),
            #[cfg(target_arch = "wasm32")]
            beginning: millisecond_to_sec(
                match web_sys::window().expect("no window").performance() {
                    Some(perf) => perf.now(),
                    None => 0.0,
                },
            ),
            current: 0.0,
            last: 0.0,
        }
    }
    /// get the current time as a marker to now
    pub fn mark(&self) -> TimeMarker {
        TimeMarker(self.current)
    }
    /// return the time since a marker
    pub fn time_since(&self, marker: TimeMarker) -> TimeDelta {
        TimeDelta(self.current - marker.0)
    }
    /// how long it has been since the last frame
    pub fn frame_diff(&self) -> TimeDelta {
        TimeDelta(self.current - self.last)
    }

    pub(crate) fn read(&mut self) -> TimeDelta {
        self.last = self.current;
        self.set_to_now();
        self.frame_diff()
    }

    pub(crate) fn set_to_now(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.current = Instant::now().duration_since(self.beginning).as_secs_f64();
        }
        #[cfg(target_arch = "wasm32")]
        {
            let now = match web_sys::window().expect("no window").performance() {
                Some(perf) => perf.now(),
                None => self.last,
            };
            self.current = millisecond_to_sec(now) - self.beginning;
        }
    }
}
/// signifies a point in time
#[derive(PartialOrd, PartialEq, Copy, Clone)]
pub struct TimeMarker(pub f64);
/// signifies a change in time
#[derive(PartialOrd, PartialEq, Copy, Clone, Default)]
pub struct TimeDelta(pub f64);

impl TimeDelta {
    pub fn as_f32(&self) -> f32 {
        self.0 as f32
    }
}

impl SubAssign for TimeDelta {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0
    }
}

impl Sub for TimeDelta {
    type Output = TimeDelta;
    fn sub(self, rhs: Self) -> Self::Output {
        TimeDelta(self.0 - rhs.0)
    }
}

impl AddAssign for TimeDelta {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl Add for TimeDelta {
    type Output = TimeDelta;
    fn add(self, rhs: Self) -> Self::Output {
        TimeDelta(self.0 + rhs.0)
    }
}

impl Div for TimeDelta {
    type Output = TimeDelta;
    fn div(self, rhs: Self) -> Self::Output {
        TimeDelta(self.0 / rhs.0)
    }
}

impl From<f32> for TimeDelta {
    fn from(value: f32) -> Self {
        Self(value as f64)
    }
}
impl From<i32> for TimeDelta {
    fn from(value: i32) -> Self {
        Self(value as f64)
    }
}
