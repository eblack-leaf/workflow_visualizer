use crate::color::Color;
use crate::coord::{Area, Depth, Position};
use crate::depth_texture::DepthTexture;
use crate::gpu_bindings::{
    TEXT_AREA, TEXT_COLOR, TEXT_DEPTH, TEXT_INSTANCE, TEXT_POSITION, TEXT_VERTEX,
    TEXT_VERTEX_POSITION,
};
use crate::viewport::ViewportBinding;
use bytemuck::{Pod, Zeroable};
use std::collections::{HashMap, HashSet};
use std::convert::Into;
use wgpu::util::DeviceExt;
use wgpu::{include_wgsl, VertexAttribute, VertexBufferLayout};
#[repr(C)]
#[derive(Pod, Zeroable, Copy, Clone)]
pub struct Instance {
    pub color: Color,
    pub position: Position,
    pub area: Area,
    pub depth: Depth,
}
impl Instance {
    pub fn attributes() -> [VertexAttribute; 4] {
        wgpu::vertex_attr_array![TEXT_COLOR => Float32x4, TEXT_POSITION => Float32x2,
            TEXT_AREA => Float32x2, TEXT_DEPTH => Float32]
    }
}
#[repr(C)]
#[derive(Pod, Zeroable, Copy, Clone)]
pub struct Vertex {
    pub position: Position,
}
impl Vertex {
    pub fn attributes<'a>() -> [VertexAttribute; 1] {
        wgpu::vertex_attr_array![TEXT_VERTEX_POSITION => Float32x2]
    }
    pub fn new<T: Into<Position>>(pos: T) -> Self {
        Self {
            position: pos.into(),
        }
    }
}
pub type GlyphHash = fontdue::layout::GlyphRasterConfig;
pub struct TextRenderer {
    pub pipeline: wgpu::RenderPipeline,
    pub vertex_buffer: wgpu::Buffer,
    pub glyph_instance_buffers: HashMap<GlyphHash, GlyphInstanceBuffer>,
}
pub struct GlyphInstanceBuffer {
    pub instance_buffer: wgpu::Buffer,
    pub instance_count: u32,
    pub instances: HashSet<Instance>,
}
impl GlyphInstanceBuffer {
    pub fn new(device: &wgpu::Device, instances: HashSet<Instance>) -> Self {
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
    pub fn update(&mut self, queue: &wgpu::Queue, instances: HashSet<Instance>) {
        queue.write_buffer(&self.instance_buffer);
    }
}
const GLYPH_AABB: [Vertex; 6] = [
    Vertex {
        position: Position { x: 0.0, y: 0.0 },
    },
    Vertex {
        position: Position { x: 0.0, y: 1.0 },
    },
    Vertex {
        position: Position { x: 1.0, y: 0.0 },
    },
    Vertex {
        position: Position { x: 1.0, y: 0.0 },
    },
    Vertex {
        position: Position { x: 0.0, y: 1.0 },
    },
    Vertex {
        position: Position { x: 1.0, y: 1.0 },
    },
];
impl TextRenderer {
    pub fn new(device: &wgpu::Device, viewport_binding: &ViewportBinding) -> Self {
        Self {
            pipeline: device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("text pipeline"),
                layout: Some(
                    &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                        label: Some("text pipeline descriptor"),
                        bind_group_layouts: &[&viewport_binding.bind_group_layout],
                        push_constant_ranges: &[],
                    }),
                ),
                vertex: wgpu::VertexState {
                    module: &device.create_shader_module(include_wgsl!("../shaders/text.wgsl")),
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
                primitive: Default::default(),
                depth_stencil: None,
                multisample: Default::default(),
                fragment: None,
                multiview: None,
            }),
            vertex_buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("text vertex buffer"),
                contents: bytemuck::cast_slice(&GLYPH_AABB),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }),
            glyph_instance_buffers: HashMap::new(),
        }
    }
    pub fn render<'a>(
        &'a self,
        mut render_pass: &mut wgpu::RenderPass<'a>,
        viewport_binding: &'a ViewportBinding,
    ) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &viewport_binding.bind_group, &[]);
        render_pass.set_vertex_buffer(TEXT_VERTEX, self.vertex_buffer.slice(..));
        self.glyph_instance_buffers.iter().for_each(
            |(glyph_id, glyph_instance_buffer): (&GlyphHash, &GlyphInstanceBuffer)| {
                render_pass.set_vertex_buffer(
                    TEXT_INSTANCE,
                    glyph_instance_buffer.instance_buffer.slice(..),
                );
                render_pass.draw(0u32..GLYPH_AABB.len(), 0..glyph_instance_buffer.instances);
            },
        );
    }
}
