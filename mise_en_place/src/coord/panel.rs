use bevy_ecs::prelude::Bundle;

use crate::coord::CoordContext;
use crate::{Depth, Section};

#[derive(Bundle, Copy, Clone, Default, Debug)]
pub struct Panel<Context: CoordContext> {
    #[bundle]
    pub section: Section<Context>,
    pub depth: Depth,
}

impl<Context: CoordContext> Panel<Context> {
    pub fn new<S: Into<Section<Context>>, D: Into<Depth>>(section: S, depth: D) -> Self {
        Self {
            section: section.into(),
            depth: depth.into(),
        }
    }
}

impl<Context: CoordContext, S: Into<Section<Context>>, D: Into<Depth>> From<(S, D)>
    for Panel<Context>
{
    fn from(value: (S, D)) -> Self {
        Panel::new(value.0.into(), value.1.into())
    }
}
