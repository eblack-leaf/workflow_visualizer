use bevy_ecs::prelude::{Commands, Component, Entity, Query};
use bytemuck::{Pod, Zeroable};

use crate::coord::Depth;

#[repr(C)]
#[derive(Component, Copy, Clone, Pod, Zeroable, Default, PartialEq)]
pub struct DepthAdjust {
    pub layer: f32,
}

impl DepthAdjust {
    pub fn new(layer: f32) -> Self {
        Self { layer }
    }
}

impl From<f32> for DepthAdjust {
    fn from(value: f32) -> Self {
        Self::new(value)
    }
}

pub(crate) fn depth_adjust(
    mut adjusted: Query<(Entity, &mut Depth, &DepthAdjust)>,
    mut cmd: Commands,
) {
    for (entity, mut depth, depth_adjust) in adjusted.iter_mut() {
        depth.adjust(*depth_adjust);
        cmd.entity(entity).remove::<DepthAdjust>();
    }
}
