use bevy_ecs::bundle::Bundle;

use crate::coord::CoordContext;
use crate::coord::layer::Layer;
use crate::coord::position::Position;

#[derive(Bundle, Copy, Clone, PartialEq)]
pub struct Location<Context: CoordContext> {
    pub position: Position<Context>,
    pub layer: Layer,
}

impl<Context: CoordContext> Location<Context> {
    pub fn new<P: Into<Position<Context>>, L: Into<Layer>>(position: P, layer: L) -> Self {
        Self {
            position: position.into(),
            layer: layer.into(),
        }
    }
}

impl<Context: CoordContext, P: Into<Position<Context>>, L: Into<Layer>> From<(P, L)>
    for Location<Context>
{
    fn from(value: (P, L)) -> Self {
        Self::new(value.0, value.1)
    }
}
