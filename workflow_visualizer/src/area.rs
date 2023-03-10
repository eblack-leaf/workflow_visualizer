use bevy_ecs::component::Component;
use std::marker::PhantomData;
use bytemuck::{Pod, Zeroable};
use crate::coord::{CoordContext, NumericalContext};

#[derive(Component, Copy, Clone)]
pub struct Area<Context: CoordContext> {
    pub width: f32,
    pub height: f32,
    _context: PhantomData<Context>,
}

impl<Context: CoordContext> Area<Context> {
    pub fn new<RA: Into<RawArea>>(width: f32, height: f32) -> Self {
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
impl<Context: CoordContext> From<(u32, u32)> for Area<Context> {
    fn from(value: (u32, u32)) -> Self {
        Area::<Context>::new(value.0 as f32, value.1 as f32)
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
