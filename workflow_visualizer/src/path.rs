use bevy_ecs::prelude::{Changed, Component, DetectChanges, IntoSystemConfigs, Query, Res};

use crate::{Attach, InterfaceContext, Position, SyncPoint, Visualizer};

/// Collection of specific points rendered from a PathView
#[derive(Component, Clone)]
pub struct Path {
    pub points: Vec<Position<InterfaceContext>>,
}

impl Path {
    pub(crate) fn new(points: Vec<Position<InterfaceContext>>) -> Path {
        Self { points }
    }
}
