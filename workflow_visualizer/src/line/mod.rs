use bevy_ecs::prelude::{Bundle, Component};
use wgpu::util::DeviceExt;

pub(crate) use attachment::LineAttachment;
pub use line_render::LineRender;

use crate::line::line_render::LineRenderPoints;
use crate::path::ResponsivePathView;
use crate::{
    Color, DeviceContext, EnableVisibility, GfxSurface, InterfaceContext, Layer, Path, Position,
    Section, Tag,
};

mod attachment;
mod line_render;
mod renderer;
mod system;
pub type LineTag = Tag<Line>;
#[derive(Bundle)]
pub struct Line {
    tag: LineTag,
    responsive_path: ResponsivePathView,
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
        responsive_path: ResponsivePathView,
        layer: L,
        color: C,
    ) -> Self {
        Self {
            tag: LineTag::new(),
            responsive_path,
            path: Path::new(vec![]),
            visibility: EnableVisibility::default(),
            section: Section::default(),
            line_render: LineRender::new(0),
            line_render_points: LineRenderPoints { points: vec![] },
            layer: layer.into(),
            color: color.into(),
        }
    }
}
