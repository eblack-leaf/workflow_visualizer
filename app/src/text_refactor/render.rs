use wgpu::include_wgsl;
use wgpu::util::DeviceExt;

use crate::coord::Position;
use crate::gpu_bindings::{bindings, buffers};
use crate::text_refactor::instance::Instance;
use crate::text_refactor::instances::Instances;
use crate::text_refactor::rasterization::Rasterization;
use crate::text_refactor::vertex::Vertex;
use crate::viewport::ViewportBinding;

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
        rasterization: &Rasterization,
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
                            &rasterization.bind_group_layout,
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
        }
    }
    pub fn render<'a>(
        &'a self,
        mut render_pass: &mut wgpu::RenderPass<'a>,
        rasterization: &'a Rasterization,
        viewport_binding: &'a ViewportBinding,
        instances: &'a Instances,
    ) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(bindings::VIEWPORT, &viewport_binding.bind_group, &[]);
        render_pass.set_bind_group(bindings::RASTERIZATION, &rasterization.bind_group, &[]);
        render_pass.set_vertex_buffer(buffers::TEXT_VERTEX, self.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(buffers::TEXT_INSTANCE, instances.instance_buffer.slice(..));
        if instances.instance_count > 0 {
            render_pass.draw(0..GLYPH_AABB.len() as u32, 0..instances.instance_count);
        }
    }
}
