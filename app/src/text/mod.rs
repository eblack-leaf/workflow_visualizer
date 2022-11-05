use bevy_ecs::prelude::{Commands, Res};

use crate::depth_texture::DepthTexture;
pub use crate::text::rasterizer::{Rasterizer, RasterizerBinding, TextFont};
pub use crate::text::renderer::TextRenderer;
use crate::viewport::ViewportBinding;

mod instance;
mod rasterizer;
mod renderer;
mod scale;
mod vertex;

pub fn setup(
    viewport_binding: Res<ViewportBinding>,
    device: Res<wgpu::Device>,
    surface_configuration: Res<wgpu::SurfaceConfiguration>,
    depth_texture: Res<DepthTexture>,
    mut cmd: Commands,
) {
    let font = TextFont::new(
        "/home/omi-voshuli/note-ifications/app/fonts/JetBrainsMono-Medium.ttf",
        13u32,
    );
    let rasterizer = Rasterizer::new();
    let rasterizer_binding = RasterizerBinding::new(&device, &rasterizer.buffer);
    let text_renderer = TextRenderer::new(
        &device,
        surface_configuration.format,
        depth_texture.format,
        &viewport_binding,
        &rasterizer_binding,
    );
    cmd.insert_resource(rasterizer_binding);
    cmd.insert_resource(text_renderer);
    cmd.insert_resource(rasterizer);
    cmd.insert_resource(font);
}
