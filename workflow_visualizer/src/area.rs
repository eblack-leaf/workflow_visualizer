use bevy_ecs::component::Component;
use std::marker::PhantomData;
use std::ops::Mul;
use bytemuck::{Pod, Zeroable};
use crate::coord::{CoordContext, NumericalContext};
use crate::{DeviceContext, InterfaceContext};

#[derive(Component, Copy, Clone, PartialOrd, PartialEq, Default)]
pub struct Area<Context: CoordContext> {
    pub width: f32,
    pub height: f32,
    _context: PhantomData<Context>,
}

impl<Context: CoordContext> Area<Context> {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            _context: PhantomData,
        }
    }
    pub fn as_numerical(&self) -> Area<NumericalContext> {
        Area::<NumericalContext>::new(self.width, self.height)
    }
    pub fn as_raw(&self) -> RawArea {
        RawArea {
            width: self.width,
            height: self.height,
        }
    }
}
impl Area<InterfaceContext> {
    pub fn to_device(&self, scale_factor: f64) -> Area<DeviceContext> {
        Area::<DeviceContext>::new(
            self.width * scale_factor as f32,
            self.height * scale_factor as f32,
        )
    }
}

impl Area<DeviceContext> {
    pub fn to_ui(&self, scale_factor: f64) -> Area<InterfaceContext> {
        Area::<InterfaceContext>::new(
            self.width / scale_factor as f32,
            self.height / scale_factor as f32,
        )
    }
}
#[repr(C)]
#[derive(Pod, Zeroable, Copy, Clone)]
pub struct RawArea {
    width: f32,
    height: f32,
}

impl RawArea {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
        }
    }
}
impl<Context: CoordContext> From<(usize, usize)> for Area<Context> {
    fn from(value: (usize, usize)) -> Self {
        Self::new(value.0 as f32, value.1 as f32)
    }
}

impl<Context: CoordContext> From<(i32, i32)> for Area<Context> {
    fn from(value: (i32, i32)) -> Self {
        Self::new(value.0 as f32, value.1 as f32)
    }
}

impl<Context: CoordContext> From<(f32, f32)> for Area<Context> {
    fn from(value: (f32, f32)) -> Self {
        Self::new(value.0 as f32, value.1 as f32)
    }
}

impl<Context: CoordContext> From<(u32, u32)> for Area<Context> {
    fn from(value: (u32, u32)) -> Self {
        Self::new(value.0 as f32, value.1 as f32)
    }
}

impl<Context: CoordContext> Mul for Area<Context> {
    type Output = Area<Context>;
    fn mul(self, rhs: Self) -> Self::Output {
        Area::<Context>::new(self.width * rhs.width, self.height * rhs.height)
    }
}
