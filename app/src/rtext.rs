use std::collections::HashMap;
use std::path::Path;

use fontdue::{Font, FontSettings, Metrics};
use fontdue::layout::{CoordinateSystem, Layout, LayoutSettings, TextStyle};
use wgpu::{include_wgsl, VertexAttribute};
use wgpu::util::DeviceExt;

use crate::color::Color;
use crate::coord::{Area, Depth, Position};
use crate::gpu_bindings::{attributes, bindings, buffers};
use crate::viewport::ViewportBinding;

#[derive(Copy, Clone)]
pub struct MaxCharacters(pub u32);

#[derive(Copy, Clone, PartialEq, Hash)]
pub struct TextScale {
    pub px: u32,
}

impl TextScale {
    pub fn px(&self) -> f32 {
        self.px as f32
    }
}

impl From<u32> for TextScale {
    fn from(px: u32) -> Self {
        Self { px }
    }
}

#[derive(Clone, Copy)]
pub struct GlyphWidth(pub f32);

pub struct TextFont {
    pub font_storage: [Font; 1],
    pub glyph_widths: HashMap<TextScale, GlyphWidth>,
}

impl TextFont {
    pub fn new<T: AsRef<Path>>(font_path: T, opt_scale: TextScale) -> Self {
        Self {
            font_storage: [Font::from_bytes(
                std::fs::read(font_path).expect("invalid font path read"),
                FontSettings {
                    scale: opt_scale.px(),
                    ..FontSettings::default()
                },
            )
                .expect("invalid font path")],
            glyph_widths: HashMap::new(),
        }
    }
    pub fn font(&self) -> &Font {
        &self.font_storage[0]
    }
    pub fn index() -> usize {
        0
    }
    pub fn font_slice(&self) -> &[Font] {
        self.font_storage.as_slice()
    }
    pub fn line_height(&self, scale: TextScale) -> f32 {
        self.font()
            .horizontal_line_metrics(scale.px())
            .expect("no line metrics")
            .new_line_size
    }
    pub fn line_width(&self, max_characters: MaxCharacters, scale: TextScale) -> f32 {
        (self.glyph_width(scale).0 as u32 * max_characters.0) as f32
    }
    pub fn glyph_width(&self, scale: TextScale) -> GlyphWidth {
        let rasterized_a = self.font().rasterize('a', scale.px());
        let glyph_width = GlyphWidth(rasterized_a.0.advance_width);
        return glyph_width;
    }
    pub fn text_line_metrics(
        &self,
        max_characters: MaxCharacters,
        scale: TextScale,
    ) -> TextLineMetrics {
        let line_height = self.line_height(scale);
        let line_width = self.line_width(max_characters, scale);
        return TextLineMetrics::new(scale, max_characters, (line_width, line_height));
    }
}

pub struct TextLineMetrics {
    pub scale: TextScale,
    pub max_characters: MaxCharacters,
    pub area: Area,
}

impl TextLineMetrics {
    pub fn new<T: Into<Area>>(scale: TextScale, max_characters: MaxCharacters, area: T) -> Self {
        Self {
            scale,
            max_characters,
            area: area.into(),
        }
    }
}

pub struct Text {
    string: String,
}

impl Text {
    pub fn new(string: String) -> Self {
        Self {
            string: string
                .lines()
                .next()
                .expect("no lines in text string input")
                .to_string()
                .replace("\n", ""),
        }
    }
    pub fn string(&self) -> &String {
        &self.string
    }
}

pub struct GlyphColorAdjustment {
    pub color: Color,
}

pub type GlyphOffset = usize;

pub struct TextLine {
    pub text: Text,
    pub base_color: Color,
    pub glyph_color_adjustments: HashMap<GlyphOffset, GlyphColorAdjustment>,
    pub text_line_metrics: TextLineMetrics,
}

impl TextLine {
    pub const ELLIPSIS: &'static str = "...";
    pub fn create_view<'a>(&self) -> TextLineView {
        let mut ellipsis_text = None;
        let mut viewable_text = Text::new("".to_string());
        if self.text.string().len() > self.text_line_metrics.max_characters.0 as usize {
            let (first, second) = self.text.string().split_at(
                (self.text_line_metrics.max_characters.0 - Self::ELLIPSIS.len() as u32) as usize,
            );
            viewable_text = Text::new(first.to_string() + Self::ELLIPSIS);
            ellipsis_text = Some(Text::new(second.to_string()));
        }
        TextLineView {
            viewable_text,
            ellipsis_text,
        }
    }
    pub fn max_characters(&self) -> usize {
        self.text_line_metrics.max_characters.0 as usize
    }
}

pub struct TextLineView {
    pub viewable_text: Text,
    pub ellipsis_text: Option<Text>,
}

pub struct TextLineStack {
    pub position: Position,
    pub depth: Depth,
    pub layout: Layout,
    pub line_stack: Vec<TextLine>,
    pub line_stack_views: Vec<TextLineView>,
}

impl TextLineStack {
    pub fn glyph_metadata(&self, raw_byte_offset: usize) -> (usize, usize) {
        let mut line_index = 0;
        let mut glyph_offset = raw_byte_offset;
        for line in &self.line_stack {
            if raw_byte_offset > line.max_characters() {
                glyph_offset -= line.max_characters();
                line_index += 1;
            } else {
                return (line_index, glyph_offset);
            }
        }
        return (line_index, glyph_offset);
    }
    pub fn instances(&self, font: &TextFont) -> Vec<Instance> {
        let mut instances = Vec::new();
        self.layout.glyphs().iter().for_each(|glyph| {
            let (line_index, glyph_offset) = self.glyph_metadata(glyph.byte_offset);
            let line = &self.line_stack[line_index];
            let color = match line.glyph_color_adjustments.get(&glyph_offset) {
                Some(adjustment) => adjustment.color,
                None => line.base_color,
            };
            instances.push(Instance::new(
                (glyph.x, glyph.y).into(),
                (glyph.width as f32, glyph.height as f32).into(),
                self.depth,
                color,
                font.rasterized_glyph_index(glyph.parent),
            ));
        });
        return instances;
    }
    pub fn new(
        font: &TextFont,
        position: Position,
        depth: Depth,
        line_stack: Vec<TextLine>,
    ) -> Self {
        let mut line_stack_views = Vec::new();
        Self {
            position,
            depth,
            layout: {
                let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
                layout.reset(&LayoutSettings {
                    x: position.x,
                    y: position.y,
                    ..LayoutSettings::default()
                });
                for line in &line_stack {
                    line_stack_views.push(line.create_view());
                    let viewable_text = &line_stack_views.last().unwrap().viewable_text;
                    let mut style_text = viewable_text.string().clone();
                    style_text.push('\n');
                    layout.append(
                        font.font_slice(),
                        &TextStyle::new(
                            style_text.as_str(),
                            line.text_line_metrics.scale.px(),
                            TextFont::index(),
                        ),
                    );
                }
                layout
            },
            line_stack,
            line_stack_views,
        }
    }
}

#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Copy, Clone)]
pub struct Vertex {
    pub position: Position,
}

impl Vertex {
    pub fn attributes<'a>() -> [VertexAttribute; 1] {
        wgpu::vertex_attr_array![attributes::TEXT_VERTEX => Float32x2]
    }
    pub const fn new(position: Position) -> Self {
        Self { position }
    }
}

const GLYPH_AABB: [Vertex; 6] = [
    Vertex::new(Position::new(0.0, 0.0)),
    Vertex::new(Position::new(0.0, 1.0)),
    Vertex::new(Position::new(1.0, 0.0)),
    Vertex::new(Position::new(1.0, 0.0)),
    Vertex::new(Position::new(0.0, 1.0)),
    Vertex::new(Position::new(1.0, 1.0)),
];

#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Copy, Clone)]
pub struct Instance {
    pub color: Color,
    pub position: Position,
    pub area: Area,
    pub depth: Depth,
    pub rasterization_buffer_index: RasterizationBufferIndex,
}

impl Instance {
    pub fn new(position: Position, area: Area, depth: Depth, color: Color, rasterization_buffer_index: RasterizationBufferIndex) -> Self {
        Self {
            position,
            area,
            depth,
            color,
            rasterization_buffer_index,
        }
    }
    pub fn attributes() -> [VertexAttribute; 5] {
        wgpu::vertex_attr_array![attributes::TEXT_COLOR => Float32x4, attributes::TEXT_POSITION => Float32x2,
            attributes::TEXT_AREA => Float32x2, attributes::TEXT_DEPTH => Float32, attributes::TEXT_RASTERIZATION_INDEX => Uint32x2]
    }
}

pub type GlyphHash = fontdue::layout::GlyphRasterConfig;
pub type Glyph = (Metrics, Vec<u8>);

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RasterizationBufferIndex {
    pub parts: [u32; 3],
}

impl RasterizationBufferIndex {
    pub fn start(&self) -> u32 {
        self.parts[0]
    }
    pub fn size(&self) -> u32 {
        self.parts[1]
    }
    pub fn rows(&self) -> u32 { self.parts[2] }
}

pub struct RasterizationBinding {
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,
}

impl RasterizationBinding {
    pub fn new(device: &wgpu::Device, cpu_buffer: &Vec<u8>) -> Self {
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("rasterizer bind group layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: bindings::RASTERIZATION,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("rasterizer buffer"),
            contents: cpu_buffer.as_slice(),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("rasterizer bind group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: bindings::RASTERIZATION,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &buffer,
                    offset: 0,
                    size: None,
                }),
            }],
        });
        Self {
            buffer,
            bind_group,
            bind_group_layout,
        }
    }
}

pub struct Rasterization {
    pub rasterized_glyphs: HashMap<GlyphHash, Glyph>,
    pub glyph_indices: HashMap<GlyphHash, RasterizationBufferIndex>,
    pub buffer: Vec<u8>,
}

impl Rasterization {
    pub fn new() -> Self {
        Self {
            rasterized_glyphs: HashMap::new(),
            glyph_indices: HashMap::new(),
            buffer: Vec::new(),
        }
    }
}

pub struct GlyphInstanceBuffer {
    pub instance_buffer: wgpu::Buffer,
    pub instance_count: u32,
    pub instances: Vec<Instance>,
}

impl GlyphInstanceBuffer {
    pub fn new(device: &wgpu::Device, instances: Vec<Instance>) -> Self {
        Self {
            instance_buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("glyph instance buffer"),
                contents: bytemuck::cast_slice(&instances),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }),
            instance_count: instances.len() as u32,
            instances,
        }
    }
}

pub struct TextRenderer {
    pub pipeline: wgpu::RenderPipeline,
    pub vertex_buffer: wgpu::Buffer,
    pub instance_buffer: GlyphInstanceBuffer,
}

impl TextRenderer {
    pub fn new(
        device: &wgpu::Device,
        surface_format: wgpu::TextureFormat,
        depth_format: wgpu::TextureFormat,
        viewport_binding: &ViewportBinding,
        rasterization_binding: &RasterizationBinding,
    ) -> Self {
        let shader = device.create_shader_module(include_wgsl!("../shaders/generated/text.wgsl"));
        Self {
            pipeline: device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("text pipeline"),
                layout: Some(
                    &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                        label: Some("text pipeline descriptor"),
                        bind_group_layouts: &[
                            &viewport_binding.bind_group_layout,
                            &rasterization_binding.bind_group_layout,
                        ],
                        push_constant_ranges: &[],
                    }),
                ),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vertex_entry",
                    buffers: &[
                        wgpu::VertexBufferLayout {
                            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                            step_mode: wgpu::VertexStepMode::Vertex,
                            attributes: Vertex::attributes().as_slice(),
                        },
                        wgpu::VertexBufferLayout {
                            array_stride: std::mem::size_of::<Instance>() as wgpu::BufferAddress,
                            step_mode: wgpu::VertexStepMode::Instance,
                            attributes: Instance::attributes().as_slice(),
                        },
                    ],
                },
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    unclipped_depth: false,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    conservative: false,
                },
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: depth_format,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::Less,
                    stencil: wgpu::StencilState::default(),
                    bias: wgpu::DepthBiasState::default(),
                }),
                multisample: Default::default(),
                fragment: Option::from(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fragment_entry",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: surface_format,
                        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                        write_mask: Default::default(),
                    })],
                }),
                multiview: None,
            }),
            vertex_buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("text vertex buffer"),
                contents: bytemuck::cast_slice(&GLYPH_AABB),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }),
            instance_buffer: GlyphInstanceBuffer::new(device, vec![]),
        }
    }
    pub fn render<'a>(
        &'a self,
        mut render_pass: &mut wgpu::RenderPass<'a>,
        rasterization_binding: &'a RasterizationBinding,
        viewport_binding: &'a ViewportBinding,
    ) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(bindings::VIEWPORT, &viewport_binding.bind_group, &[]);
        render_pass.set_bind_group(
            bindings::RASTERIZATION,
            &rasterization_binding.bind_group,
            &[],
        );
        render_pass.set_vertex_buffer(buffers::TEXT_VERTEX, self.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(
            buffers::TEXT_INSTANCE,
            self.instance_buffer.instance_buffer.slice(..),
        );
        render_pass.draw(
            0..GLYPH_AABB.len() as u32,
            0..self.instance_buffer.instance_count,
        );
    }
}
