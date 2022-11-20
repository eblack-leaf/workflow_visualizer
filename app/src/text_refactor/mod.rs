use std::collections::HashMap;

use bevy_ecs::prelude::{Bundle, Commands, Component, Res};

use crate::color::Color;
use crate::coord::Panel;
use crate::depth_texture::DepthTexture;
use crate::text_refactor::font::Font;
use crate::text_refactor::glyph_cache::GlyphCache;
pub use crate::text_refactor::glyph_cache::RemovedRasterizations;
pub use crate::text_refactor::instances::Instances;
pub use crate::text_refactor::rasterization::Rasterizations;
pub use crate::text_refactor::rasterizer::RasterizationReferences;
use crate::text_refactor::rasterizer::Rasterizer;
pub use crate::text_refactor::render::TextRenderer;
use crate::viewport::ViewportBinding;

mod font;
mod glyph_cache;
pub mod glyphs;
mod instance;
pub mod instances;
mod placement;
pub mod rasterization;
mod rasterization_descriptor;
mod rasterizer;
pub mod render;
mod scale;
mod vertex;

#[derive(Component)]
pub struct Text {
    pub text: String,
}

pub struct ColorAdjustment {
    pub adjustments: HashMap<usize, Color>,
}

#[derive(Bundle)]
pub struct TextBundle {
    pub text: Text,
    #[bundle]
    pub panel: Panel,
    pub base_color: Color,
}

pub fn render_setup(
    viewport_binding: Res<ViewportBinding>,
    device: Res<wgpu::Device>,
    surface_configuration: Res<wgpu::SurfaceConfiguration>,
    depth_texture: Res<DepthTexture>,
    mut cmd: Commands,
) {
    let font = Font::new(
        "/home/omi-voshuli/note-ifications/app/fonts/JetBrainsMono-Medium.ttf",
        13u32,
    );
    let rasterizer = Rasterizer::new();
    let rasterization = Rasterizations::new(&device, 1024);
    let text_renderer = TextRenderer::new(
        &device,
        surface_configuration.format,
        depth_texture.format,
        &viewport_binding,
        &rasterization,
    );
    let instances = Instances::new(&device, 1024);
    cmd.insert_resource(instances);
    cmd.insert_resource(rasterization);
    cmd.insert_resource(text_renderer);
    cmd.insert_resource(rasterizer);
    cmd.insert_resource(font);
}

pub fn compute_setup(mut cmd: Commands) {
    let glyph_cache = GlyphCache::new();
    cmd.insert_resource(glyph_cache);
}
