use crate::coord::{Area, Depth, Panel, Position};
use bevy_ecs::prelude::{Commands, Component, Res, ResMut};
use std::collections::HashMap;
use std::num::NonZeroU32;
use crate::color::Color;
use crate::uniform::Uniform;
use crate::viewport::ViewportBinding;

#[derive(Component, Clone)]
pub struct Text {
    pub text: String,
}
pub struct Font {
    pub font: fontdue::Font,
    pub cache: HashMap<char, GlyphData>,
    pub scale: f32,
}
impl Font {
    pub fn new(data: &[u8], scale: f32) -> Self {
        Self {
            font: fontdue::Font::from_bytes(
                data,
                fontdue::FontSettings {
                    scale,
                    ..fontdue::FontSettings::default()
                },
            )
            .expect("could not build font out of given font bytes"),
            cache: HashMap::new(),
            scale,
        }
    }
    pub fn horizontal_line_size(&self) -> f32 {
        if let Some(metrics) = self.font.horizontal_line_metrics(self.scale) {
            return metrics.new_line_size;
        }
        return self.scale;
    }
    pub fn cache_glyph_data(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, character: char)  {
        if self.cache.contains_key(&character) { return }
        let (metrics, bitmap) = self.font.rasterize(character, self.scale);
        let extent = wgpu::Extent3d{
            width: metrics.width as u32,
            height: metrics.height as u32,
            ..wgpu::Extent3d::default()
        };
        let texture = device.create_texture(
            &wgpu::TextureDescriptor{
                label: Some(format!("glyph data: {:?}", character).as_str()),
                size: extent,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::R8Unorm,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST
            }
        );
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        queue.write_texture(
            texture.as_image_copy(),
            bytemuck::cast_slice(&bitmap),
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(NonZeroU32(metrics.width as u32)),
                rows_per_image: None
            },
            extent
        );
        self.cache.insert( character, GlyphData::new (
            character,
            texture,
            texture_view,
            metrics
        ));
    }
}
#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Copy, Clone)]
pub struct VertexData {
    pub position: Position,
}

impl VertexData {
    pub fn desc() -> () {

    }
}
#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Copy, Clone)]
pub struct InstanceData {
    pub texture_index: u32,
    pub position: Position,
    pub area: Area,
    pub depth: Depth,
    pub color: Color,
}

impl InstanceData {
    pub fn desc() -> () {

    }
}
pub struct Renderer {
    pub pipeline: wgpu::RenderPipeline,
    pub vertex_buffer: wgpu::Buffer,
    pub instance_buffer: wgpu::Buffer,
}
pub fn setup(
    device: Res<wgpu::Device>,
    queue: Res<wgpu::Queue>,
    viewport_binding: Res<ViewportBinding>,
    mut cmd: Commands,
) {
    let sampler = device.create_sampler(&wgpu::SamplerDescriptor::default());
}
pub fn render<'a>(
    mut render_pass: ResMut<wgpu::RenderPass<'a>>,

) {
}
pub struct GlyphData {
    pub character: char,
    pub texture: wgpu::Texture,
    pub texture_view: wgpu::TextureView,
    pub metrics: fontdue::Metrics,
}
impl GlyphData {
    pub fn new(character: char, texture: wgpu::Texture, texture_view: wgpu::TextureView, metrics: fontdue::Metrics) -> Self {
        Self {
            character, texture, texture_view, metrics
        }
    }
}
