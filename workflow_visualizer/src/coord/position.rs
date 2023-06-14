use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Div, Sub};

use bevy_ecs::component::Component;
use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};

use crate::{DeviceContext, InterfaceContext};
use crate::coord::{CoordinateContext, NumericalContext};

/// Position denotes 2d coordinates in space with float32 precision
#[derive(Component, Copy, Clone, PartialOrd, PartialEq, Default)]
pub struct Position<Context: CoordinateContext> {
    pub x: f32,
    pub y: f32,
    _context: PhantomData<Context>,
}
impl<Context: CoordinateContext> Debug for Position<Context> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Position: x:{:.2} y:{:.2}", self.x, self.y)
    }
}
impl<Context: CoordinateContext> Position<Context> {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            _context: PhantomData,
        }
    }
    /// returns a copy as just a number.
    pub fn as_numerical(&self) -> Position<NumericalContext> {
        Position::<NumericalContext>::new(self.x, self.y)
    }
    /// returns a copy as a raw position
    pub fn as_raw(&self) -> RawPosition {
        RawPosition {
            x: self.x,
            y: self.y,
        }
    }
}
impl Position<InterfaceContext> {
    /// useful for converting to a device position accounting for scale factor
    pub fn to_device(&self, scale_factor: f64) -> Position<DeviceContext> {
        Position::<DeviceContext>::new(self.x * scale_factor as f32, self.y * scale_factor as f32)
    }
}

impl Position<DeviceContext> {
    /// converts to interface context accounting for scale factor
    pub fn to_interface(&self, scale_factor: f64) -> Position<InterfaceContext> {
        Position::<InterfaceContext>::new(
            self.x / scale_factor as f32,
            self.y / scale_factor as f32,
        )
    }
}
impl<Context: CoordinateContext> Add for Position<Context> {
    type Output = Position<Context>;
    fn add(self, rhs: Self) -> Self::Output {
        Position::<Context>::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<Context: CoordinateContext> Sub for Position<Context> {
    type Output = Position<Context>;
    fn sub(self, rhs: Self) -> Self::Output {
        Position::<Context>::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<Context: CoordinateContext> Div for Position<Context> {
    type Output = Position<Context>;

    fn div(self, rhs: Self) -> Self::Output {
        Position::<Context>::new(self.x / rhs.x, self.y / rhs.y)
    }
}
/// Raw position for interacting with C
#[repr(C)]
#[derive(Pod, Zeroable, Copy, Clone, Default, Serialize, Deserialize, Debug)]
pub struct RawPosition {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

impl RawPosition {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}
impl<Context: CoordinateContext> From<(f32, f32)> for Position<Context> {
    fn from(value: (f32, f32)) -> Self {
        Position::<Context>::new(value.0, value.1)
    }
}

impl<Context: CoordinateContext> From<(f64, f64)> for Position<Context> {
    fn from(value: (f64, f64)) -> Self {
        Position::<Context>::new(value.0 as f32, value.1 as f32)
    }
}

impl<Context: CoordinateContext> From<(u32, u32)> for Position<Context> {
    fn from(value: (u32, u32)) -> Self {
        Position::<Context>::new(value.0 as f32, value.1 as f32)
    }
}

impl<Context: CoordinateContext> From<(i32, i32)> for Position<Context> {
    fn from(value: (i32, i32)) -> Self {
        Position::<Context>::new(value.0 as f32, value.1 as f32)
    }
}

impl<Context: CoordinateContext> From<(usize, usize)> for Position<Context> {
    fn from(value: (usize, usize)) -> Self {
        Position::<Context>::new(value.0 as f32, value.1 as f32)
    }
}
impl<Context: CoordinateContext> AddAssign for Position<Context> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
