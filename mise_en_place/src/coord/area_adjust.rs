use std::marker::PhantomData;

use bevy_ecs::prelude::{Commands, Component, Entity, Query};

use crate::coord::{Area, CoordContext};

#[derive(Component, Copy, Clone, Default, PartialEq)]
pub struct AreaAdjust<Context: CoordContext> {
    pub width: f32,
    pub height: f32,
    _context: PhantomData<Context>,
}

impl<Context: CoordContext> AreaAdjust<Context> {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            _context: PhantomData,
        }
    }
}

impl<Context: CoordContext> From<(f32, f32)> for AreaAdjust<Context> {
    fn from(value: (f32, f32)) -> Self {
        Self::new(value.0, value.1)
    }
}

pub(crate) fn area_adjust<Context: CoordContext>(
    mut adjusted: Query<(Entity, &mut Area<Context>, &AreaAdjust<Context>)>,
    mut cmd: Commands,
) {
    for (entity, mut area, area_adjust) in adjusted.iter_mut() {
        area.adjust(*area_adjust);
        cmd.entity(entity).remove::<AreaAdjust<Context>>();
    }
}
