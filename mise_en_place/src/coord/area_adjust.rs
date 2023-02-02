use bevy_ecs::prelude::{Commands, Component, Entity, Query};
use bytemuck::{Pod, Zeroable};

use crate::coord::Area;

#[repr(C)]
#[derive(Component, Copy, Clone, Pod, Zeroable, Default, PartialEq)]
pub struct AreaAdjust {
    pub width: f32,
    pub height: f32,
}

impl AreaAdjust {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

impl From<(f32, f32)> for AreaAdjust {
    fn from(value: (f32, f32)) -> Self {
        Self::new(value.0, value.1)
    }
}

pub(crate) fn area_adjust(
    mut adjusted: Query<(Entity, &mut Area, &AreaAdjust)>,
    mut cmd: Commands,
) {
    for (entity, mut area, area_adjust) in adjusted.iter_mut() {
        area.adjust(*area_adjust);
        cmd.entity(entity).remove::<AreaAdjust>();
    }
}

#[repr(C)]
#[derive(Component, Copy, Clone, Pod, Zeroable, Default, PartialEq)]
pub struct ScaledAreaAdjust {
    pub width: f32,
    pub height: f32,
}

impl ScaledAreaAdjust {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

impl From<(f32, f32)> for ScaledAreaAdjust {
    fn from(value: (f32, f32)) -> Self {
        Self::new(value.0, value.1)
    }
}
