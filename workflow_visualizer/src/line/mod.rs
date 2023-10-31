use bevy_ecs::prelude::Bundle;

pub(crate) use attachment::LineAttachment;
pub use line_render::LineRender;

use crate::line::line_render::LineRenderPoints;
use crate::{Color, EnableVisibility, InterfaceContext, Layer, Path, Position, Section, Tag};

mod attachment;
mod line_render;
mod renderer;
mod system;
pub type LineTag = Tag<Line>;
#[derive(Bundle)]
pub struct Line {
    tag: LineTag,
    path: Path,
    visibility: EnableVisibility,
    section: Section<InterfaceContext>,
    line_render: LineRender,
    line_render_points: LineRenderPoints,
    layer: Layer,
    color: Color,
}

impl Line {
    pub fn new<L: Into<Layer>, C: Into<Color>>(
        path_points: Vec<Position<InterfaceContext>>,
        layer: L,
        color: C,
    ) -> Self {
        let capacity = path_points.len().checked_sub(1).unwrap_or_default();
        Self {
            tag: LineTag::new(),
            path: Path::new(path_points),
            visibility: EnableVisibility::default(),
            section: Section::default(),
            line_render: LineRender::new(capacity),
            line_render_points: LineRenderPoints { points: vec![] },
            layer: layer.into(),
            color: color.into(),
        }
    }
}
