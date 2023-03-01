use std::ops::{Add, AddAssign, Div, Sub, SubAssign};

use bevy_ecs::prelude::{ResMut, Resource};
#[cfg(not(target_arch = "wasm32"))]
use tokio::time::Instant;

use crate::{Attach, Engen, FrontEndStages};

#[derive(Resource)]
pub struct Timer {
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) beginning: Instant,
    pub current: f64,
    pub last: f64,
}

impl Timer {
    pub(crate) fn new() -> Self {
        Self {
            #[cfg(not(target_arch = "wasm32"))]
            beginning: Instant::now(),
            current: 0.0,
            last: 0.0,
        }
    }
    pub fn mark(&self) -> TimeMarker {
        TimeMarker(self.current)
    }
    pub fn time_since(&self, marker: TimeMarker) -> TimeDelta {
        TimeDelta(self.current - marker.0)
    }
    pub fn frame_diff(&self) -> TimeDelta {
        TimeDelta(self.current - self.last)
    }
    pub(crate) fn read(&mut self) -> TimeDelta {
        self.last = self.current;
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.current = Instant::now().duration_since(self.beginning).as_secs_f64();
        }
        #[cfg(target_arch = "wasm32")]
        {
            self.current = match web_sys::window().expect("no window").performance() {
                Some(perf) => perf.now(),
                None => self.last,
            }
        }
        self.frame_diff()
    }
}

pub(crate) fn read_time(mut timer: ResMut<Timer>) {
    let _delta = timer.read();
}

impl Attach for Timer {
    fn attach(engen: &mut Engen) {
        engen.frontend.container.insert_resource(Timer::new());
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::First, read_time);
    }
}

#[derive(PartialOrd, PartialEq, Copy, Clone)]
pub struct TimeMarker(pub f64);

#[derive(PartialOrd, PartialEq, Copy, Clone)]
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