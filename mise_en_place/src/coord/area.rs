use bevy_ecs::component::Component;
use bytemuck::{Pod, Zeroable};

use crate::coord::area_adjust::{AreaAdjust, ScaledAreaAdjust};

#[repr(C)]
#[derive(Pod, Zeroable, Component, Clone, Copy, Default, PartialEq, Debug)]
pub struct Area {
    pub width: f32,
    pub height: f32,
}

impl Area {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
    pub fn to_scaled(&self, scale_factor: f64) -> ScaledArea {
        ScaledArea::new(
            self.width * scale_factor as f32,
            self.height * scale_factor as f32,
        )
    }
    pub fn adjust<Adjust: Into<AreaAdjust>>(&mut self, adjust: Adjust) {
        let adjust = adjust.into();
        self.width += adjust.width;
        self.height += adjust.height;
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

impl From<(i32, i32)> for Area {
    fn from(value: (i32, i32)) -> Self {
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
#[derive(Component, Pod, Zeroable, Clone, Copy, Default, PartialEq, Debug)]
pub struct ScaledArea {
    pub width: f32,
    pub height: f32,
}

impl ScaledArea {
    pub fn to_area(&self, scale_factor: f64) -> Area {
        Area::new(
            self.width / scale_factor as f32,
            self.height / scale_factor as f32,
        )
    }
    pub fn as_area(&self) -> Area {
        Area::new(self.width, self.height)
    }
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
    pub fn adjust<Adjust: Into<ScaledAreaAdjust>>(&mut self, adjust: Adjust) {
        let adjust = adjust.into();
        self.width += adjust.width;
        self.height += adjust.height;
    }
}

impl From<(usize, usize)> for ScaledArea {
    fn from(value: (usize, usize)) -> Self {
        Self {
            width: value.0 as f32,
            height: value.1 as f32,
        }
    }
}

impl From<(u32, u32)> for ScaledArea {
    fn from(value: (u32, u32)) -> Self {
        Self {
            width: value.0 as f32,
            height: value.1 as f32,
        }
    }
}

impl From<(f32, f32)> for ScaledArea {
    fn from(value: (f32, f32)) -> Self {
        Self {
            width: value.0 as f32,
            height: value.1 as f32,
        }
    }
}
