use std::marker::PhantomData;

use bevy_ecs::component::Component;
use bytemuck::{Pod, Zeroable};

use crate::coord::{CoordContext, Device, Logical, View};
use crate::coord::area_adjust::AreaAdjust;

#[repr(C)]
#[derive(Pod, Zeroable, Copy, Clone, Default)]
pub struct GpuArea {
    pub width: f32,
    pub height: f32,
}

#[derive(Component, Clone, Copy, Default, PartialEq, Debug)]
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
    pub fn adjust<Adjust: Into<AreaAdjust<Context>>>(&mut self, adjust: Adjust) {
        let adjust = adjust.into();
        self.width += adjust.width;
        self.height += adjust.height;
    }
    pub fn as_logical(&self) -> Area<Logical> {
        Area::<Logical>::new(self.width, self.height)
    }
    pub fn to_gpu(&self) -> GpuArea {
        GpuArea {
            width: self.width,
            height: self.height,
        }
    }
}

impl Area<View> {
    pub fn to_device(&self, scale_factor: f64) -> Area<Device> {
        Area::<Device>::new(
            self.width * scale_factor as f32,
            self.height * scale_factor as f32,
        )
    }
}

impl Area<Device> {
    pub fn to_view(&self, scale_factor: f64) -> Area<View> {
        Area::<View>::new(
            self.width / scale_factor as f32,
            self.height / scale_factor as f32,
        )
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
