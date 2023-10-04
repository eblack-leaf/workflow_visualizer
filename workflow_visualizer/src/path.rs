use bevy_ecs::prelude::{Changed, Component, DetectChanges, IntoSystemConfigs, Query, Res};

use crate::{Attach, GridLocation, InterfaceContext, Position, SyncPoint, Visualizer};

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

/// Collection of PathViewPoints
#[derive(Clone, Component)]
pub struct PathView {
    pub points: Vec<GridLocation>,
}

impl<T: Into<GridLocation>> From<Vec<T>> for PathView {
    fn from(mut value: Vec<T>) -> Self {
        Self {
            points: value.drain(..).map(|v| v.into()).collect(),
        }
    }
}
