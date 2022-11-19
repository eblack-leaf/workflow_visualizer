use crate::color::Color;
use crate::coord::{Area, Depth, Position};
use crate::gpu_bindings::bindings;
use crate::text_step_out::attributes::{AttributeBuffer, Coordinator};
use crate::text_step_out::rasterization::placement::RasterizationPlacement;
use crate::text_step_out::vertex::Vertex;
use crate::viewport::ViewportBinding;
use wgpu::util::DeviceExt;
use wgpu::{include_wgsl, VertexAttribute};
const GLYPH_AABB: [Vertex; 6] = [
    Vertex::new(Position::new(0.0, 0.0)),
    Vertex::new(Position::new(0.0, 1.0)),
    Vertex::new(Position::new(1.0, 0.0)),
    Vertex::new(Position::new(1.0, 0.0)),
    Vertex::new(Position::new(0.0, 1.0)),
    Vertex::new(Position::new(1.0, 1.0)),
];

pub struct TextRenderer {
    pub pipeline: wgpu::RenderPipeline,
    pub vertex_buffer: wgpu::Buffer,
}

impl TextRenderer {
    pub fn new(
        device: &wgpu::Device,
        surface_format: wgpu::TextureFormat,
        depth_format: wgpu::TextureFormat,
        viewport_binding: &ViewportBinding,
        rasterizations: &Rasterizations,
    ) -> Self {
        let shader = device.create_shader_module(include_wgsl!("text.wgsl"));
        Self {
            pipeline: device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("text pipeline"),
                layout: Some(
                    &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                        label: Some("text pipeline descriptor"),
                        bind_group_layouts: &[
                            &viewport_binding.bind_group_layout,
                            &rasterizations.bind_group_layout,
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
                            array_stride: std::mem::size_of::<Position>() as wgpu::BufferAddress,
                            step_mode: wgpu::VertexStepMode::Instance,
                            attributes: &*wgpu::vertex_attr_array![0 => Float32x2],
                        },
                        wgpu::VertexBufferLayout {
                            array_stride: std::mem::size_of::<Area>() as wgpu::BufferAddress,
                            step_mode: wgpu::VertexStepMode::Instance,
                            attributes: &*wgpu::vertex_attr_array![1 => Float32x2],
                        },
                        wgpu::VertexBufferLayout {
                            array_stride: std::mem::size_of::<Depth>() as wgpu::BufferAddress,
                            step_mode: wgpu::VertexStepMode::Instance,
                            attributes: &*wgpu::vertex_attr_array![2 => Float32],
                        },
                        wgpu::VertexBufferLayout {
                            array_stride: std::mem::size_of::<Color>() as wgpu::BufferAddress,
                            step_mode: wgpu::VertexStepMode::Instance,
                            attributes: &*wgpu::vertex_attr_array![3 => Float32x4],
                        },
                        wgpu::VertexBufferLayout {
                            array_stride: std::mem::size_of::<RasterizationPlacement>()
                                as wgpu::BufferAddress,
                            step_mode: wgpu::VertexStepMode::Instance,
                            attributes: &*wgpu::vertex_attr_array![4 => Uint32x3],
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
        }
    }
    pub fn render<'a>(
        &'a self,
        mut render_pass: &mut wgpu::RenderPass<'a>,
        rasterization: &'a Rasterizations,
        viewport_binding: &'a ViewportBinding,
        coordinator: &'a Coordinator,
        positions: &'a AttributeBuffer<Position>,
        areas: &'a AttributeBuffer<Area>,
        depths: &'a AttributeBuffer<Depth>,
        colors: &'a AttributeBuffer<Color>,
        rasterization_placements: &'a AttributeBuffer<RasterizationPlacement>,
    ) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(bindings::VIEWPORT, &viewport_binding.bind_group, &[]);
        render_pass.set_bind_group(bindings::RASTERIZATION, &rasterization.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, positions.attributes.slice(..));
        render_pass.set_vertex_buffer(2, areas.attributes.slice(..));
        render_pass.set_vertex_buffer(3, depths.attributes.slice(..));
        render_pass.set_vertex_buffer(4, colors.attributes.slice(..));
        render_pass.set_vertex_buffer(5, rasterization_placements.attributes.slice(..));
        if coordinator.current > 0 {
            render_pass.draw(0..GLYPH_AABB.len() as u32, 0..&coordinator.current as u32);
        }
    }
}
