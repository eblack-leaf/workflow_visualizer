use std::marker::PhantomData;

use bevy_ecs::prelude::{Commands, Component, Entity, Query};

use crate::coord::{CoordContext, Position};

#[derive(Component, Copy, Clone, Default, PartialEq)]
pub struct PositionAdjust<Context: CoordContext> {
    pub x: f32,
    pub y: f32,
    _context: PhantomData<Context>,
}

impl<Context: CoordContext> PositionAdjust<Context> {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            _context: PhantomData,
        }
    }
}

impl<Context: CoordContext> From<(f32, f32)> for PositionAdjust<Context> {
    fn from(value: (f32, f32)) -> Self {
        Self::new(value.0, value.1)
    }
}

pub(crate) fn position_adjust<Context: CoordContext>(
    mut adjusted: Query<(Entity, &mut Position<Context>, &PositionAdjust<Context>), ()>,
    mut cmd: Commands,
) {
    for (entity, mut position, position_adjust) in adjusted.iter_mut() {
        position.adjust(*position_adjust);
        cmd.entity(entity).remove::<PositionAdjust<Context>>();
    }
}
