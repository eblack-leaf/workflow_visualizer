use std::marker::PhantomData;
use std::ops::Sub;

use bevy_ecs::component::Component;
use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};

use crate::coord::{CoordContext, DeviceView, Numerical, PositionAdjust, UIView};

#[derive(Component, Copy, Clone, Default, PartialEq, Debug)]
pub struct Position<Context: CoordContext> {
    pub x: f32,
    pub y: f32,
    _coord_context: PhantomData<Context>,
}

#[repr(C)]
#[derive(Pod, Zeroable, Copy, Clone, Default, Serialize, Deserialize)]
pub struct GpuPosition {
    pub x: f32,
    pub y: f32,
}

impl<Context: CoordContext> Sub for Position<Context> {
    type Output = Position<Context>;
    fn sub(self, rhs: Self) -> Self::Output {
        Position::<Context>::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<Context: CoordContext> Position<Context> {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            _coord_context: PhantomData,
        }
    }
    pub(crate) fn adjust<Adjust: Into<PositionAdjust<Context>>>(&mut self, adjust: Adjust) {
        let adjust = adjust.into();
        self.x += adjust.x;
        self.y += adjust.y;
    }
    pub fn as_numerical(&self) -> Position<Numerical> {
        Position::<Numerical>::new(self.x, self.y)
    }
    pub fn to_gpu(&self) -> GpuPosition {
        GpuPosition {
            x: self.x,
            y: self.y,
        }
    }
}

impl Position<UIView> {
    pub fn to_device(&self, scale_factor: f64) -> Position<DeviceView> {
        Position::<DeviceView>::new(self.x * scale_factor as f32, self.y * scale_factor as f32)
    }
}

impl Position<DeviceView> {
    pub fn to_ui(&self, scale_factor: f64) -> Position<UIView> {
        Position::<UIView>::new(self.x / scale_factor as f32, self.y / scale_factor as f32)
    }
}

impl<Context: CoordContext> From<(f32, f32)> for Position<Context> {
    fn from(value: (f32, f32)) -> Self {
        Self {
            x: value.0,
            y: value.1,
            _coord_context: PhantomData,
        }
    }
}

impl<Context: CoordContext> From<(f64, f64)> for Position<Context> {
    fn from(value: (f64, f64)) -> Self {
        Self {
            x: value.0 as f32,
            y: value.1 as f32,
            _coord_context: PhantomData,
        }
    }
}

impl<Context: CoordContext> From<(u32, u32)> for Position<Context> {
    fn from(value: (u32, u32)) -> Self {
        Self {
            x: value.0 as f32,
            y: value.1 as f32,
            _coord_context: PhantomData,
        }
    }
}

impl<Context: CoordContext> From<(i32, i32)> for Position<Context> {
    fn from(value: (i32, i32)) -> Self {
        Self {
            x: value.0 as f32,
            y: value.1 as f32,
            _coord_context: PhantomData,
        }
    }
}

impl<Context: CoordContext> From<(usize, usize)> for Position<Context> {
    fn from(value: (usize, usize)) -> Self {
        Self {
            x: value.0 as f32,
            y: value.1 as f32,
            _coord_context: PhantomData,
        }
    }
}
