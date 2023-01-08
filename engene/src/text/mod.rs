mod attribute;
mod font;
mod rasterization;
mod scale;
mod vertex;

use crate::canvas::Viewport;
use crate::color::Color;
use crate::coord::{Area, Depth, Position};
use crate::instance::EntityKey;
use crate::render::{Render, RenderPassHandle, RenderPhase};
use crate::task::Stage;
use crate::text::scale::Scale;
use crate::text::vertex::{Vertex, GLYPH_AABB};
use crate::{instance, Attach, Canvas, Engen, Task};
use bevy_ecs::prelude::{Commands, Res, ResMut, Resource};
use wgpu::{include_wgsl, VertexAttribute, VertexFormat};
pub(crate) type GlyphHash = fontdue::layout::GlyphRasterConfig;
#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub struct GlyphOffset(pub usize);
pub struct Request {
    pub character: char,
    pub scale: Scale,
    pub hash: GlyphHash,
    pub position: Position,
    pub area: Area,
    pub depth: Depth,
    pub color: Color,
    pub descriptor: Option<rasterization::Descriptor>,
}
pub(crate) type InstanceCoordinator = instance::Coordinator<EntityKey<GlyphOffset>, Request>;
#[derive(Resource)]
pub struct TextRenderer {
    pub(crate) pipeline: wgpu::RenderPipeline,
    pub(crate) coordinator: InstanceCoordinator,
    pub(crate) vertex_buffer: wgpu::Buffer,
    pub(crate) rasterization: rasterization::Handler,
}
impl TextRenderer {
    pub(crate) fn read_rasterization_requests(&mut self) {
        self.rasterization.read_requests(&self.coordinator);
    }
    pub(crate) fn integrate_rasterization_descriptors(&mut self) {
        self.rasterization.integrate_requests(&mut self.coordinator);
    }
    pub(crate) fn new(canvas: &Canvas) -> Self {
        let rasterization = rasterization::Handler::new(&canvas.device);
        Self {
            pipeline: {
                let shader = canvas
                    .device
                    .create_shader_module(include_wgsl!("text.wgsl"));
                let pipeline = canvas.device.create_render_pipeline(
                    &wgpu::RenderPipelineDescriptor {
                        label: Some("text pipeline"),
                        layout: Some(&canvas.device.create_pipeline_layout(
                            &wgpu::PipelineLayoutDescriptor {
                                label: Some("text pipeline descriptor"),
                                bind_group_layouts: &[
                                    &canvas.viewport.bind_group_layout,
                                    &rasterization.binding.bind_group_layout,
                                ],
                                push_constant_ranges: &[],
                            },
                        )),
                        vertex: wgpu::VertexState {
                            module: &shader,
                            entry_point: "vertex_entry",
                            buffers: &[
                                wgpu::VertexBufferLayout {
                                    array_stride: std::mem::size_of::<Vertex>()
                                        as wgpu::BufferAddress,
                                    step_mode: wgpu::VertexStepMode::Vertex,
                                    attributes: Vertex::attributes().as_slice(),
                                },
                                wgpu::VertexBufferLayout {
                                    array_stride: std::mem::size_of::<Position>()
                                        as wgpu::BufferAddress,
                                    step_mode: wgpu::VertexStepMode::Instance,
                                    attributes: &[VertexAttribute {
                                        format: VertexFormat::Float32x2,
                                        offset: 0,
                                        shader_location: 1,
                                    }],
                                },
                                wgpu::VertexBufferLayout {
                                    array_stride: std::mem::size_of::<Area>()
                                        as wgpu::BufferAddress,
                                    step_mode: wgpu::VertexStepMode::Instance,
                                    attributes: &[VertexAttribute {
                                        format: VertexFormat::Float32x2,
                                        offset: 0,
                                        shader_location: 2,
                                    }],
                                },
                                wgpu::VertexBufferLayout {
                                    array_stride: std::mem::size_of::<Depth>()
                                        as wgpu::BufferAddress,
                                    step_mode: wgpu::VertexStepMode::Instance,
                                    attributes: &[VertexAttribute {
                                        format: VertexFormat::Float32,
                                        offset: 0,
                                        shader_location: 3,
                                    }],
                                },
                                wgpu::VertexBufferLayout {
                                    array_stride: std::mem::size_of::<Color>()
                                        as wgpu::BufferAddress,
                                    step_mode: wgpu::VertexStepMode::Instance,
                                    attributes: &[VertexAttribute {
                                        format: VertexFormat::Float32x4,
                                        offset: 0,
                                        shader_location: 4,
                                    }],
                                },
                                wgpu::VertexBufferLayout {
                                    array_stride: std::mem::size_of::<rasterization::Descriptor>()
                                        as wgpu::BufferAddress,
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
                            format: canvas.viewport.depth_format,
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
                                format: canvas.surface_configuration.format,
                                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                                write_mask: Default::default(),
                            })],
                        }),
                        multiview: None,
                    },
                );
                pipeline
            },
            coordinator: {
                let mut coordinator = instance::Coordinator::new();
                // setup attributes
                coordinator
            },
            vertex_buffer: vertex::buffer(&canvas.device),
            rasterization,
        }
    }
}
pub fn startup(canvas: Res<Canvas>, mut cmd: Commands) {
    cmd.insert_resource(TextRenderer::new(&canvas));
}
pub fn prepare(canvas: Res<Canvas>, mut renderer: ResMut<TextRenderer>) {
    renderer.read_rasterization_requests();
    // ...
    renderer.integrate_rasterization_descriptors();
    renderer.coordinator.prepare();
    renderer.coordinator.process();
    renderer.coordinator.finish();
}
impl Attach for TextRenderer {
    fn attach(engen: &mut Engen) {
        engen
            .render
            .startup
            .schedule
            .add_system_to_stage(Stage::During, startup);
        engen
            .render
            .main
            .schedule
            .add_system_to_stage(Stage::During, prepare);
    }
}
impl Render for TextRenderer {
    fn extract(compute: &Task, render: &mut Task)
    where
        Self: Sized,
    {
        todo!()
    }

    fn phase() -> RenderPhase {
        RenderPhase::Alpha
    }

    fn render<'a>(&'a self, render_pass_handle: &mut RenderPassHandle<'a>, viewport: &'a Viewport) {
        render_pass_handle.0.set_pipeline(&self.pipeline);
        render_pass_handle
            .0
            .set_bind_group(0, &viewport.bind_group, &[]);
        render_pass_handle
            .0
            .set_bind_group(1, &self.rasterization.binding.bind_group, &[]);
        render_pass_handle
            .0
            .set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass_handle
            .0
            .set_vertex_buffer(1, self.coordinator.gpu_buffer::<Position>().slice(..));
        render_pass_handle
            .0
            .set_vertex_buffer(2, self.coordinator.gpu_buffer::<Area>().slice(..));
        render_pass_handle
            .0
            .set_vertex_buffer(3, self.coordinator.gpu_buffer::<Depth>().slice(..));
        render_pass_handle
            .0
            .set_vertex_buffer(4, self.coordinator.gpu_buffer::<Color>().slice(..));
        render_pass_handle.0.set_vertex_buffer(
            5,
            self.coordinator
                .gpu_buffer::<rasterization::Descriptor>()
                .slice(..),
        );
        if self.coordinator.current() > 0 {
            render_pass_handle.0.draw(
                0..GLYPH_AABB.len() as u32,
                0..self.coordinator.current() as u32,
            );
        }
    }
}
