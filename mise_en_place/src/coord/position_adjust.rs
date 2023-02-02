use bevy_ecs::prelude::{Commands, Component, Entity, Query};
use bytemuck::{Pod, Zeroable};

use crate::coord::Position;

#[repr(C)]
#[derive(Component, Copy, Clone, Pod, Zeroable, Default, PartialEq)]
pub struct PositionAdjust {
    pub x: f32,
    pub y: f32,
}

impl PositionAdjust {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub fn to_scaled(&self, scale_factor: f64) -> ScaledPositionAdjust {
        ScaledPositionAdjust::new(self.x * scale_factor as f32, self.y * scale_factor as f32)
    }
}

impl From<(f32, f32)> for PositionAdjust {
    fn from(value: (f32, f32)) -> Self {
        Self::new(value.0, value.1)
    }
}

pub(crate) fn position_adjust(
    mut adjusted: Query<(Entity, &mut Position, &PositionAdjust), ()>,
    mut cmd: Commands,
) {
    for (entity, mut position, position_adjust) in adjusted.iter_mut() {
        position.adjust(*position_adjust);
        cmd.entity(entity).remove::<PositionAdjust>();
    }
}

#[repr(C)]
#[derive(Component, Copy, Clone, Pod, Zeroable, Default, PartialEq)]
pub struct ScaledPositionAdjust {
    pub x: f32,
    pub y: f32,
}

impl ScaledPositionAdjust {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl From<(f32, f32)> for ScaledPositionAdjust {
    fn from(value: (f32, f32)) -> Self {
        Self::new(value.0, value.1)
    }
}
