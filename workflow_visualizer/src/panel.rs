use bevy_ecs::bundle::Bundle;
use crate::coord::CoordContext;
use crate::layer::Layer;
use crate::section::Section;

#[derive(Bundle, Copy, Clone)]
pub struct Panel<Context: CoordContext> {
    #[bundle]
    pub section: Section<Context>,
    pub layer: Layer,
}

impl<Context: CoordContext> Panel<Context> {
    pub fn new<S: Into<Section<Context>>, L: Into<Layer>>(section: S, layer: L) -> Self {
        Self {
            section: section.into(),
            layer: layer.into(),
        }
    }
}
