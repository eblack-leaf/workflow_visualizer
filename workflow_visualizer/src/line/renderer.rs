use std::collections::HashMap;

use bevy_ecs::prelude::{Commands, Entity, Res, Resource};
use tracing::trace;
use wgpu::util::DeviceExt;

use crate::gfx::{GfxSurfaceConfiguration, MsaaRenderAdapter};
use crate::{
    Color, DeviceContext, GfxSurface, Layer, Position, RawPosition, Render, RenderPassHandle,
    RenderPhase, Uniform, Viewport,
};

pub(crate) struct LineRenderGroup {
    pub(crate) line_render_gpu: LineRenderGpu,
    pub(crate) capacity: usize,
    pub(crate) color: Color,
    pub(crate) color_dirty: bool,
    pub(crate) layer_and_hooks: LayerAndHooks,
    pub(crate) layer_and_hooks_dirty: bool,
    pub(crate) color_uniform: Uniform<Color>,
    pub(crate) layer_and_hooks_uniform: Uniform<LayerAndHooks>,
    bind_group: wgpu::BindGroup,
}

impl LineRenderGroup {
    pub(crate) fn new(
        line_render_gpu: LineRenderGpu,
        capacity: usize,
        layer_and_hooks: LayerAndHooks,
        color: Color,
        gfx: &GfxSurface,
        bind_group_layout: &wgpu::BindGroupLayout,
    ) -> LineRenderGroup {
        let color_uniform = Uniform::new(&gfx.device, color);
        let layer_and_hooks_uniform = Uniform::new(&gfx.device, layer_and_hooks);
        let bind_group = gfx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("line render group"),
            layout: bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: color_uniform.buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: layer_and_hooks_uniform.buffer.as_entire_binding(),
                },
            ],
        });
        Self {
            line_render_gpu,
            capacity,
            color,
            color_dirty: false,
            layer_and_hooks,
            layer_and_hooks_dirty: false,
            color_uniform,
            layer_and_hooks_uniform,
            bind_group,
        }
    }
}

#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Copy, Clone)]
pub(crate) struct LayerAndHooks {
    pub(crate) aspects: [f32; 4],
}

impl LayerAndHooks {
    pub(crate) fn new(layer: f32, aspect1: f32, aspect2: f32, aspect3: f32) -> Self {
        Self {
            aspects: [layer, aspect1, aspect2, aspect3],
        }
    }
}

#[derive(Resource)]
pub(crate) struct LineRenderer {
    pipeline: wgpu::RenderPipeline,
    pub(crate) render_groups: HashMap<Entity, LineRenderGroup>,
    pub(crate) bind_group_layout: wgpu::BindGroupLayout,
}

pub(crate) fn setup(
    gfx: Res<GfxSurface>,
    gfx_config: Res<GfxSurfaceConfiguration>,
    msaa: Res<MsaaRenderAdapter>,
    viewport: Res<Viewport>,
    mut cmd: Commands,
) {
    trace!("setup line renderer");
    let line_renderer_bind_group_layout =
        gfx.device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("line renderer bind group"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
            });
    let layout_descriptor = wgpu::PipelineLayoutDescriptor {
        label: Some("line renderer layout"),
        bind_group_layouts: &[
            &viewport.bind_group_layout(),
            &line_renderer_bind_group_layout,
        ],
        push_constant_ranges: &[],
    };
    let layout = gfx.device.create_pipeline_layout(&layout_descriptor);
    let shader = gfx
        .device
        .create_shader_module(wgpu::include_wgsl!("line.wgsl"));
    let vertex_state = wgpu::VertexState {
        module: &shader,
        entry_point: "vertex_entry",
        buffers: &[wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<RawPosition>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &wgpu::vertex_attr_array![0 => Float32x2],
        }],
    };
    let primitive_state = wgpu::PrimitiveState {
        topology: wgpu::PrimitiveTopology::LineStrip,
        front_face: wgpu::FrontFace::Ccw,
        ..wgpu::PrimitiveState::default()
    };
    let depth_stencil_state = Some(wgpu::DepthStencilState {
        format: viewport.depth_format(),
        depth_write_enabled: true,
        depth_compare: wgpu::CompareFunction::Less,
        stencil: Default::default(),
        bias: Default::default(),
    });
    let fragment_state = wgpu::FragmentState {
        module: &shader,
        entry_point: "fragment_entry",
        targets: &[Some(wgpu::ColorTargetState {
            format: gfx_config.configuration.format,
            blend: None,
            write_mask: Default::default(),
        })],
    };
    let pipeline_descriptor = wgpu::RenderPipelineDescriptor {
        label: Some("line renderer"),
        layout: Some(&layout),
        vertex: vertex_state,
        primitive: primitive_state,
        depth_stencil: depth_stencil_state,
        multisample: msaa.multisample_state(),
        fragment: Some(fragment_state),
        multiview: None,
    };
    let pipeline = gfx.device.create_render_pipeline(&pipeline_descriptor);
    let line_renderer = LineRenderer {
        pipeline,
        render_groups: HashMap::new(),
        bind_group_layout: line_renderer_bind_group_layout,
    };
    cmd.insert_resource(line_renderer);
}

impl Render for LineRenderer {
    fn phase() -> RenderPhase {
        RenderPhase::Opaque
    }

    fn render<'a>(&'a self, render_pass_handle: &mut RenderPassHandle<'a>, viewport: &'a Viewport) {
        for render_group in self.render_groups.values() {
            if render_group.capacity > 0 {
                render_pass_handle.0.set_pipeline(&self.pipeline);
                render_pass_handle
                    .0
                    .set_bind_group(0, &viewport.bind_group(), &[]);
                render_pass_handle
                    .0
                    .set_bind_group(1, &render_group.bind_group, &[]);
                render_pass_handle
                    .0
                    .set_vertex_buffer(0, render_group.line_render_gpu.buffer.slice(..));
                render_pass_handle
                    .0
                    .draw(0u32..render_group.capacity as u32 + 1, 0..1);
            }
        }
    }
}

pub(crate) struct LineRenderGpu {
    pub(crate) buffer: wgpu::Buffer,
}

impl LineRenderGpu {
    pub(crate) fn new(gfx: &GfxSurface, points: &Vec<Position<DeviceContext>>) -> Self {
        Self {
            buffer: gfx
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("line render gpu buffer"),
                    contents: bytemuck::cast_slice(
                        points
                            .iter()
                            .map(|p| p.as_raw())
                            .collect::<Vec<RawPosition>>()
                            .as_slice(),
                    ),
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                }),
        }
    }
}
