use std::ops::Sub;
use bytemuck::{Pod, Zeroable};
use bevy_ecs::component::Component;
#[repr(C)]
#[derive(Pod, Zeroable, Component, Copy, Clone, Default, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub(crate) fn to_gpu(&self, scale_factor: f64) -> GpuPosition {
        GpuPosition::new(self.x * scale_factor as f32, self.y * scale_factor as f32)
    }
}

impl From<(f32, f32)> for Position {
    fn from(value: (f32, f32)) -> Self {
        Self {
            x: value.0 as f32,
            y: value.1 as f32,
        }
    }
}

impl From<(u32, u32)> for Position {
    fn from(value: (u32, u32)) -> Self {
        Self {
            x: value.0 as f32,
            y: value.1 as f32,
        }
    }
}

impl From<(usize, usize)> for Position {
    fn from(value: (usize, usize)) -> Self {
        Self {
            x: value.0 as f32,
            y: value.1 as f32,
        }
    }
}

#[repr(C)]
#[derive(Pod, Zeroable, Copy, Clone)]
pub(crate) struct GpuPosition {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

impl GpuPosition {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub fn as_pos(&self) -> Position {
        Position::new(self.x, self.y)
    }
}
impl Sub for GpuPosition {
    type Output = GpuPosition;
    fn sub(self, rhs: Self) -> Self::Output {
        GpuPosition::new(self.x - rhs.x, self.y - rhs.y)
    }
}
impl From<(f32, f32)> for GpuPosition {
    fn from(value: (f32, f32)) -> Self {
        Self {
            x: value.0 as f32,
            y: value.1 as f32,
        }
    }
}

impl From<(u32, u32)> for GpuPosition {
    fn from(value: (u32, u32)) -> Self {
        Self {
            x: value.0 as f32,
            y: value.1 as f32,
        }
    }
}

impl From<(usize, usize)> for GpuPosition {
    fn from(value: (usize, usize)) -> Self {
        Self {
            x: value.0 as f32,
            y: value.1 as f32,
        }
    }
}
