use bevy_ecs::component::Component;
use bytemuck::{Pod, Zeroable};
#[repr(C)]
#[derive(Pod, Zeroable, Component, Clone, Copy, Default, PartialEq)]
pub struct Area {
    pub width: f32,
    pub height: f32,
}

impl Area {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
    pub(crate) fn to_gpu(&self, scale_factor: f64) -> GpuArea {
        GpuArea::new(
            self.width * scale_factor as f32,
            self.height * scale_factor as f32,
        )
    }
}

impl From<(usize, usize)> for Area {
    fn from(value: (usize, usize)) -> Self {
        Self {
            width: value.0 as f32,
            height: value.1 as f32,
        }
    }
}

impl From<(u32, u32)> for Area {
    fn from(value: (u32, u32)) -> Self {
        Self {
            width: value.0 as f32,
            height: value.1 as f32,
        }
    }
}

impl From<(f32, f32)> for Area {
    fn from(value: (f32, f32)) -> Self {
        Self {
            width: value.0 as f32,
            height: value.1 as f32,
        }
    }
}

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy)]
pub(crate) struct GpuArea {
    pub(crate) width: f32,
    pub(crate) height: f32,
}

impl GpuArea {
    pub(crate) fn as_area(&self, scale_factor: f64) -> Area {
        Area::new(
            self.width / scale_factor as f32,
            self.height / scale_factor as f32,
        )
    }
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

impl From<(usize, usize)> for GpuArea {
    fn from(value: (usize, usize)) -> Self {
        Self {
            width: value.0 as f32,
            height: value.1 as f32,
        }
    }
}

impl From<(u32, u32)> for GpuArea {
    fn from(value: (u32, u32)) -> Self {
        Self {
            width: value.0 as f32,
            height: value.1 as f32,
        }
    }
}

impl From<(f32, f32)> for GpuArea {
    fn from(value: (f32, f32)) -> Self {
        Self {
            width: value.0 as f32,
            height: value.1 as f32,
        }
    }
}
