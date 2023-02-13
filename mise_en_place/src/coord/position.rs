use std::marker::PhantomData;
use std::ops::Sub;

use bevy_ecs::component::Component;
use bytemuck::{Pod, Zeroable};

use crate::coord::{CoordContext, Logical, PositionAdjust, Scaled, Unscaled};

#[derive(Component, Copy, Clone, Default, PartialEq, Debug)]
pub struct Position<Context: CoordContext> {
    pub x: f32,
    pub y: f32,
    _coord_context: PhantomData<Context>,
}

#[repr(C)]
#[derive(Pod, Zeroable, Copy, Clone, Default)]
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
        Self { x, y, _coord_context: PhantomData }
    }
    pub(crate) fn adjust<Adjust: Into<PositionAdjust<Context>>>(&mut self, adjust: Adjust) {
        let adjust = adjust.into();
        self.x += adjust.x;
        self.y += adjust.y;
    }
    pub fn as_logical(&self) -> Position::<Logical> {
        Position::<Logical>::new(self.x, self.y)
    }
    pub fn to_gpu(&self) -> GpuPosition {
        GpuPosition {
            x: self.x,
            y: self.y,
        }
    }
}

impl Position<Unscaled> {
    pub fn to_scaled(&self, scale_factor: f64) -> Position<Scaled> {
        Position::<Scaled>::new(self.x * scale_factor as f32, self.y * scale_factor as f32)
    }
}

impl Position<Scaled> {
    pub fn to_unscaled(&self, scale_factor: f64) -> Position<Unscaled> {
        Position::<Unscaled>::new(self.x / scale_factor as f32, self.y / scale_factor as f32)
    }
}

impl<Context: CoordContext> From<(f32, f32)> for Position<Context> {
    fn from(value: (f32, f32)) -> Self {
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
