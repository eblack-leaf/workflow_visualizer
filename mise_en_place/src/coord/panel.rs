use crate::coord::CoordContext;
use crate::{Depth, Section};
use bevy_ecs::prelude::Bundle;

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
impl<Context: CoordContext, S: Into<Section<Context>>, D: Into<Depth>> From<(Context, S, D)>
    for Panel<Context>
{
    fn from(value: (Context, S, D)) -> Self {
        Panel::new(value.1.into(), value.2.into())
    }
}
