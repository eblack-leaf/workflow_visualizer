use bevy_ecs::prelude::Bundle;

use crate::{Depth, Position};
use crate::coord::CoordContext;

#[derive(Bundle, Copy, Clone, PartialEq)]
pub struct Location<Context: CoordContext> {
    pub position: Position<Context>,
    pub depth: Depth,
}

impl<Context: CoordContext> Location<Context> {
    pub fn new<P: Into<Position<Context>>, D: Into<Depth>>(position: P, depth: D) -> Self {
        Self {
            position: position.into(),
            depth: depth.into(),
        }
    }
}

impl<Context: CoordContext, P: Into<Position<Context>>, D: Into<Depth>> From<(P, D)>
for Location<Context>
{
    fn from(value: (P, D)) -> Self {
        Self::new(value.0, value.1)
    }
}
