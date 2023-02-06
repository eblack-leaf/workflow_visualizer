use std::ops::Sub;

use bevy_ecs::component::Component;
use bytemuck::{Pod, Zeroable};

use crate::coord::position_adjust::ScaledPositionAdjust;
use crate::coord::PositionAdjust;

#[repr(C)]
#[derive(Pod, Zeroable, Component, Copy, Clone, Default, PartialEq, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub fn to_scaled(&self, scale_factor: f64) -> ScaledPosition {
        ScaledPosition::new(self.x * scale_factor as f32, self.y * scale_factor as f32)
    }
    pub(crate) fn adjust<Adjust: Into<PositionAdjust>>(&mut self, adjust: Adjust) {
        let adjust = adjust.into();
        self.x += adjust.x;
        self.y += adjust.y;
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
#[derive(Component, Pod, Zeroable, Copy, Clone, Default, PartialEq, Debug)]
pub struct ScaledPosition {
    pub x: f32,
    pub y: f32,
}

impl ScaledPosition {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub fn to_pos(&self, scale_factor: f64) -> Position {
        Position::new(self.x / scale_factor as f32, self.y / scale_factor as f32)
    }
    pub fn as_pos(&self) -> Position {
        Position::new(self.x, self.y)
    }
    #[allow(unused)]
    pub(crate) fn adjust<Adjust: Into<ScaledPositionAdjust>>(&mut self, adjust: Adjust) {
        let adjust = adjust.into();
        self.x += adjust.x;
        self.y += adjust.y;
    }
}

impl Sub for ScaledPosition {
    type Output = ScaledPosition;
    fn sub(self, rhs: Self) -> Self::Output {
        ScaledPosition::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl From<(f32, f32)> for ScaledPosition {
    fn from(value: (f32, f32)) -> Self {
        Self {
            x: value.0 as f32,
            y: value.1 as f32,
        }
    }
}

impl From<(u32, u32)> for ScaledPosition {
    fn from(value: (u32, u32)) -> Self {
        Self {
            x: value.0 as f32,
            y: value.1 as f32,
        }
    }
}

impl From<(usize, usize)> for ScaledPosition {
    fn from(value: (usize, usize)) -> Self {
        Self {
            x: value.0 as f32,
            y: value.1 as f32,
        }
    }
}
