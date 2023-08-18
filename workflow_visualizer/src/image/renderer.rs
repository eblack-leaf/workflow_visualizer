use std::collections::HashMap;

use bevy_ecs::prelude::{Commands, Component, Entity, Res, Resource};
use wgpu::util::DeviceExt;

use crate::{GfxSurface, GfxSurfaceConfiguration, MsaaRenderAdapter, RawPosition, Render, RenderPassHandle, RenderPhase, TextureAtlas, TextureBindGroup, Uniform, Viewport};
use crate::image::render_group::ImageRenderGroup;

#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Copy, Clone)]
pub(crate) struct Vertex {
    pub(crate) data: [f32; 3],
}

impl Vertex {
    pub(crate) const fn new(position: RawPosition, fade: f32) -> Self {
        Self {
            data: [position.x, position.y, fade],
        }
    }
}

pub(crate) const AABB: [Vertex; 6] = [
    Vertex::new(RawPosition { x: 0.0, y: 0.0 }, 0.9),
    Vertex::new(RawPosition { x: 0.0, y: 1.0 }, 0.5),
    Vertex::new(RawPosition { x: 1.0, y: 0.0 }, 0.5),
    Vertex::new(RawPosition { x: 1.0, y: 0.0 }, 0.5),
    Vertex::new(RawPosition { x: 0.0, y: 1.0 }, 0.5),
    Vertex::new(RawPosition { x: 1.0, y: 1.0 }, 0.2),
];

pub(crate) fn aabb_vertex_buffer(gfx_surface: &GfxSurface) -> wgpu::Buffer {
    gfx_surface
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("image vertex buffer"),
            contents: bytemuck::cast_slice(&AABB),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        })
}

#[repr(C)]
#[derive(
bytemuck::Pod, bytemuck::Zeroable, Component, Copy, Clone, PartialOrd, PartialEq, Default, Debug,
)]
pub struct ImageFade(pub f32);

#[derive(Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct ImageName(pub &'static str);

pub struct ImageRequest {
    pub name: ImageName,
    pub data:,
}

#[derive(Resource)]
pub(crate) struct ImageRenderer {
    pub(crate) pipeline: wgpu::RenderPipeline,
    pub(crate) render_groups: HashMap<Entity, ImageRenderGroup>,
    pub(crate) vertex_buffer: wgpu::Buffer,
    pub(crate) sampler_bind_group: wgpu::BindGroup,
    pub(crate) images: HashMap<ImageName, TextureAtlas>,
}

pub(crate) fn setup_renderer(
    mut cmd: Commands,
    gfx: Res<GfxSurface>,
    msaa: Res<MsaaRenderAdapter>,
    viewport: Res<Viewport>,
    gfx_config: Res<GfxSurfaceConfiguration>,
) {
    let sampler_bind_group_layout_descriptor = wgpu::BindGroupLayoutDescriptor {
        label: Some("sampler bind group layout"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
            count: None,
        }],
    };
    let sampler_bind_group_layout = gfx
        .device
        .create_bind_group_layout(&sampler_bind_group_layout_descriptor);
    let sampler_descriptor = wgpu::SamplerDescriptor {
        label: Some("image sampler"),
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Linear,
        mipmap_filter: wgpu::FilterMode::Linear,
        lod_min_clamp: Default::default(),
        lod_max_clamp: Default::default(),
        compare: None,
        anisotropy_clamp: 1,
        border_color: None,
    };
    let sampler = gfx.device.create_sampler(&sampler_descriptor);
    let sampler_bind_group_descriptor = wgpu::BindGroupDescriptor {
        label: Some("sampler bind group"),
        layout: &sampler_bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: wgpu::BindingResource::Sampler(&sampler),
        }],
    };
    let sampler_bind_group = gfx.device.create_bind_group(&sampler_bind_group_descriptor);
    let render_group_layout =
        gfx.device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("image-render-group-layout"),
                entries: &[TextureBindGroup::entry(0)],
            });
    let fade_layout = gfx
        .device
        .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("fade"),
            entries: &[Uniform::vertex_bind_group_entry(0)],
        });
    let pipeline_layout_descriptor = wgpu::PipelineLayoutDescriptor {
        label: Some("image-render-pipeline-layout"),
        bind_group_layouts: &[
            viewport.bind_group_layout(),
            &sampler_bind_group_layout,
            &render_group_layout,
            &fade_layout,
        ],
        push_constant_ranges: &[],
    };
    let pipeline_layout = gfx
        .device
        .create_pipeline_layout(&pipeline_layout_descriptor);
    let shader = gfx
        .device
        .create_shader_module(wgpu::include_wgsl!("image.wgsl"));
    let pipeline_descriptor = wgpu::RenderPipelineDescriptor {
        label: Some("image renderer"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vertex_entry",
            buffers: &[wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &wgpu::vertex_attr_array![0 => Float32x3],
            }],
        },
        primitive: gfx.filled_triangle_list(),
        depth_stencil: Some(viewport.depth_stencil_state()),
        multisample: msaa.multisample_state(),
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fragment_entry",
            targets: &[Some(gfx_config.alpha_color_target_state())],
        }),
        multiview: None,
    };
    let pipeline = gfx.device.create_render_pipeline(&pipeline_descriptor);
    let renderer = ImageRenderer {
        pipeline,
        render_groups: HashMap::new(),
        vertex_buffer: aabb_vertex_buffer(&gfx),
        sampler_bind_group,
    };
    cmd.insert_resource(renderer);
}

impl Render for ImageRenderer {
    fn phase() -> RenderPhase {
        RenderPhase::Alpha(7)
    }

    fn render<'a>(&'a self, render_pass_handle: &mut RenderPassHandle<'a>, viewport: &'a Viewport) {
        render_pass_handle
            .0
            .set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass_handle
            .0
            .set_bind_group(0, viewport.bind_group(), &[]);
        render_pass_handle
            .0
            .set_bind_group(1, &self.sampler_bind_group, &[]);
        for (_, group) in self.render_groups.iter() {
            render_pass_handle
                .0
                .set_bind_group(2, &group.image_bind_group.bind_group, &[]);
            render_pass_handle
                .0
                .set_bind_group(3, &group.fade_bind_group, &[]);
            render_pass_handle.0.draw(0..AABB.len() as u32, 0..1);
        }
    }
}
