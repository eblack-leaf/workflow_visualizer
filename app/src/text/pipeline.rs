use crate::depth_texture::DepthTexture;
use crate::text::vertex::Vertex;
use crate::text::Placement;
use crate::viewport::ViewportBinding;
use crate::{Area, Color, Depth, Position};
use bevy_ecs::prelude::{Commands, Res};
use wgpu::{include_wgsl, VertexAttribute, VertexFormat};

pub struct Pipeline {
    pub pipeline: wgpu::RenderPipeline,
}
pub fn setup(
    mut cmd: Commands,
    device: Res<wgpu::Device>,
    viewport_binding: Res<ViewportBinding>,
    depth_texture: Res<DepthTexture>,
    surface_configuration: Res<wgpu::SurfaceConfiguration>,
) {
    let shader = device.create_shader_module(include_wgsl!("text.wgsl"));
    let pipeline = Pipeline {
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
                        attributes: &[VertexAttribute {
                            format: VertexFormat::Float32x2,
                            offset: 0,
                            shader_location: 1,
                        }],
                    },
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<Area>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &[VertexAttribute {
                            format: VertexFormat::Float32x2,
                            offset: 0,
                            shader_location: 2,
                        }],
                    },
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<Depth>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &[VertexAttribute {
                            format: VertexFormat::Float32,
                            offset: 0,
                            shader_location: 3,
                        }],
                    },
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<Color>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &[VertexAttribute {
                            format: VertexFormat::Float32x4,
                            offset: 0,
                            shader_location: 4,
                        }],
                    },
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<Placement>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &[VertexAttribute {
                            format: VertexFormat::Uint32x3,
                            offset: 0,
                            shader_location: 5,
                        }],
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
                format: depth_texture.format,
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
                    format: surface_configuration.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: Default::default(),
                })],
            }),
            multiview: None,
        }),
    };
    cmd.insert_resource(pipeline);
}
