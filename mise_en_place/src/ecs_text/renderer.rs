use std::collections::{HashMap, HashSet};

use bevy_ecs::prelude::{Commands, Entity, IntoSystemDescriptor, Res, Resource, SystemLabel};

use crate::ecs_text::atlas::AtlasBindGroup;
use crate::ecs_text::backend_system::{
    create_render_groups, render_group_differences, reset_extraction, resize_receiver,
};
use crate::ecs_text::coords::Coords;
use crate::ecs_text::extraction::Extraction;
use crate::ecs_text::glyph::Key;
use crate::ecs_text::gpu_buffer::GpuBuffer;
use crate::ecs_text::index::Indexer;
use crate::ecs_text::null_bit::NullBit;
use crate::ecs_text::render_group::{DrawSection, RenderGroupBindGroup};
use crate::ecs_text::renderer::TextSystems::{CreateRenderGroups, RenderGroupDiff};
use crate::ecs_text::scale::AlignedFonts;
use crate::ecs_text::vertex::{vertex_buffer, Vertex, GLYPH_AABB};
use crate::extract::Extract;
use crate::gfx::{GfxSurface, GfxSurfaceConfiguration};
use crate::job::{Container, Task};
use crate::render::{Render, RenderPassHandle, RenderPhase};
use crate::viewport::Viewport;
use crate::window::ScaleFactor;
use crate::{Area, Attach, BackEndStartupStages, BackendStages, Job, Position, Stove};

#[derive(Resource)]
pub struct TextRenderer {
    pub(crate) pipeline: wgpu::RenderPipeline,
    pub(crate) vertex_buffer: wgpu::Buffer,
    pub(crate) sampler: wgpu::Sampler,
    pub(crate) sampler_bind_group: wgpu::BindGroup,
    pub(crate) render_groups: HashMap<Entity, Entity>,
    pub(crate) render_group_bind_group_layout: wgpu::BindGroupLayout,
    pub(crate) atlas_bind_group_layout: wgpu::BindGroupLayout,
    pub(crate) container: Container,
}

pub(crate) fn setup(
    gfx_surface: Res<GfxSurface>,
    gfx_surface_config: Res<GfxSurfaceConfiguration>,
    viewport: Res<Viewport>,
    mut cmd: Commands,
    scale_factor: Res<ScaleFactor>,
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
    let sampler_bind_group_layout = gfx_surface
        .device
        .create_bind_group_layout(&sampler_bind_group_layout_descriptor);
    let sampler_descriptor = wgpu::SamplerDescriptor {
        label: Some("text sampler"),
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Linear,
        mipmap_filter: wgpu::FilterMode::Linear,
        lod_min_clamp: Default::default(),
        lod_max_clamp: Default::default(),
        compare: None,
        anisotropy_clamp: None,
        border_color: None,
    };
    let sampler = gfx_surface.device.create_sampler(&sampler_descriptor);
    let sampler_bind_group_descriptor = wgpu::BindGroupDescriptor {
        label: Some("sampler bind group"),
        layout: &sampler_bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: wgpu::BindingResource::Sampler(&sampler),
        }],
    };
    let sampler_bind_group = gfx_surface
        .device
        .create_bind_group(&sampler_bind_group_descriptor);
    let render_group_bind_group_layout_descriptor = wgpu::BindGroupLayoutDescriptor {
        label: Some("rasterization bind group"),
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
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
    };
    let render_group_bind_group_layout = gfx_surface
        .device
        .create_bind_group_layout(&render_group_bind_group_layout_descriptor);
    let atlas_bind_group_layout_descriptor = wgpu::BindGroupLayoutDescriptor {
        label: Some("atlas bind group layout descriptor"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Texture {
                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                view_dimension: wgpu::TextureViewDimension::D2,
                multisampled: false,
            },
            count: None,
        }],
    };
    let atlas_bind_group_layout = gfx_surface
        .device
        .create_bind_group_layout(&atlas_bind_group_layout_descriptor);
    let layout_descriptor = wgpu::PipelineLayoutDescriptor {
        label: Some("text pipeline layout descriptor"),
        bind_group_layouts: &[
            &viewport.bind_group_layout,
            &sampler_bind_group_layout,
            &render_group_bind_group_layout,
            &atlas_bind_group_layout,
        ],
        push_constant_ranges: &[],
    };
    let layout = gfx_surface
        .device
        .create_pipeline_layout(&layout_descriptor);
    let shader = gfx_surface
        .device
        .create_shader_module(wgpu::include_wgsl!("padded_text.wgsl"));
    let vertex_state = wgpu::VertexState {
        module: &shader,
        entry_point: "vertex_entry",
        buffers: &[
            wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &wgpu::vertex_attr_array![0 => Float32x2],
            },
            wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<Position>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Instance,
                attributes: &wgpu::vertex_attr_array![1 => Float32x2],
            },
            wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<NullBit>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Instance,
                attributes: &wgpu::vertex_attr_array![2 => Uint32],
            },
            wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<Coords>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Instance,
                attributes: &wgpu::vertex_attr_array![3 => Float32x4],
            },
            wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<Area>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Instance,
                attributes: &wgpu::vertex_attr_array![4 => Float32x2],
            },
        ],
    };
    let primitive_state = wgpu::PrimitiveState {
        topology: wgpu::PrimitiveTopology::TriangleList,
        strip_index_format: None,
        front_face: wgpu::FrontFace::Ccw,
        cull_mode: Some(wgpu::Face::Back),
        unclipped_depth: false,
        polygon_mode: wgpu::PolygonMode::Fill,
        conservative: false,
    };
    let depth_stencil_state = Some(wgpu::DepthStencilState {
        format: viewport.depth_format,
        depth_write_enabled: true,
        depth_compare: wgpu::CompareFunction::Less,
        stencil: wgpu::StencilState::default(),
        bias: wgpu::DepthBiasState::default(),
    });
    let fragment_state = wgpu::FragmentState {
        module: &shader,
        entry_point: "fragment_entry",
        targets: &[Some(wgpu::ColorTargetState {
            format: gfx_surface_config.configuration.format,
            blend: Some(wgpu::BlendState::ALPHA_BLENDING),
            write_mask: Default::default(),
        })],
    };
    let descriptor = wgpu::RenderPipelineDescriptor {
        label: Some("text pipeline"),
        layout: Some(&layout),
        vertex: vertex_state,
        primitive: primitive_state,
        depth_stencil: depth_stencil_state,
        multisample: wgpu::MultisampleState::default(),
        fragment: Some(fragment_state),
        multiview: None,
    };
    let pipeline = gfx_surface.device.create_render_pipeline(&descriptor);
    let mut renderer = TextRenderer {
        pipeline,
        vertex_buffer: vertex_buffer(&gfx_surface),
        sampler,
        sampler_bind_group,
        render_groups: HashMap::new(),
        render_group_bind_group_layout,
        atlas_bind_group_layout,
        container: Container::new(),
    };
    cmd.insert_resource(renderer);
    cmd.insert_resource(Extraction::new());
    cmd.insert_resource(AlignedFonts::new(scale_factor.factor));
}

#[derive(SystemLabel)]
enum TextSystems {
    CreateRenderGroups,
    RenderGroupDiff,
}

impl Attach for TextRenderer {
    fn attach(stove: &mut Stove) {
        stove
            .backend
            .startup
            .add_system_to_stage(BackEndStartupStages::Setup, setup);
        stove.backend.main.add_system_to_stage(
            BackendStages::Prepare,
            create_render_groups.label(CreateRenderGroups),
        );
        stove.backend.main.add_system_to_stage(
            BackendStages::Prepare,
            render_group_differences
                .label(RenderGroupDiff)
                .after(CreateRenderGroups),
        );
        stove.backend.main.add_system_to_stage(
            BackendStages::Prepare,
            resize_receiver.after(RenderGroupDiff),
        );
        stove
            .backend
            .main
            .add_system_to_stage(BackendStages::Last, reset_extraction);
    }
}

impl Extract for TextRenderer {
    fn extract(frontend: &mut Job, backend: &mut Job)
    where
        Self: Sized,
    {
        let mut extraction = frontend
            .container
            .get_resource_mut::<Extraction>()
            .expect("no extraction in compute");
        backend.container.insert_resource(extraction.clone());
        *extraction = Extraction::new();
    }
}

impl Render for TextRenderer {
    fn phase() -> RenderPhase {
        RenderPhase::Alpha
    }
    fn render<'a>(&'a self, render_pass_handle: &mut RenderPassHandle<'a>, viewport: &'a Viewport) {
        render_pass_handle.0.set_pipeline(&self.pipeline);
        render_pass_handle
            .0
            .set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass_handle
            .0
            .set_bind_group(0, &viewport.bind_group, &[]);
        render_pass_handle
            .0
            .set_bind_group(1, &self.sampler_bind_group, &[]);
        for (_, render_group) in self.render_groups.iter() {
            let indexer = self
                .container
                .get::<Indexer<Key>>(*render_group)
                .expect("no indexer");
            if indexer.count() > 0 {
                let glyph_positions = self
                    .container
                    .get::<GpuBuffer<Position>>(*render_group)
                    .expect("no glyph position buffer");
                render_pass_handle
                    .0
                    .set_vertex_buffer(1, glyph_positions.buffer.slice(..));
                let glyph_areas = self
                    .container
                    .get::<GpuBuffer<Area>>(*render_group)
                    .expect("no glyph area buffer");
                render_pass_handle
                    .0
                    .set_vertex_buffer(4, glyph_areas.buffer.slice(..));
                let null_bits = self
                    .container
                    .get::<GpuBuffer<NullBit>>(*render_group)
                    .expect("no null bits buffer");
                render_pass_handle
                    .0
                    .set_vertex_buffer(2, null_bits.buffer.slice(..));
                let coords = self
                    .container
                    .get::<GpuBuffer<Coords>>(*render_group)
                    .expect("no coords buffer");
                render_pass_handle
                    .0
                    .set_vertex_buffer(3, coords.buffer.slice(..));
                let render_group_bind_group = self
                    .container
                    .get::<RenderGroupBindGroup>(*render_group)
                    .expect("");
                render_pass_handle
                    .0
                    .set_bind_group(2, &render_group_bind_group.bind_group, &[]);
                let atlas_bind_group = self
                    .container
                    .get::<AtlasBindGroup>(*render_group)
                    .expect("");
                render_pass_handle
                    .0
                    .set_bind_group(3, &atlas_bind_group.bind_group, &[]);
                let draw_section = self.container.get::<DrawSection>(*render_group).expect("");
                if let Some(scissor_rect) = draw_section.section {
                    render_pass_handle.0.set_scissor_rect(
                        scissor_rect.position.x as u32,
                        scissor_rect.position.y as u32,
                        scissor_rect.area.width as u32,
                        scissor_rect.area.height as u32,
                    );
                }
                render_pass_handle
                    .0
                    .draw(0..GLYPH_AABB.len() as u32, 0..indexer.count());
                if let Some(_) = draw_section.section {
                    render_pass_handle.0.set_scissor_rect(
                        0,
                        0,
                        viewport.cpu.area.width as u32,
                        viewport.cpu.area.height as u32,
                    )
                }
            }
        }
    }
}
