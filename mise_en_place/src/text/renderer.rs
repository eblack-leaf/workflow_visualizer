use std::collections::HashMap;

use bevy_ecs::prelude::{
    Commands, Entity, IntoSystemDescriptor, Res, Resource, StageLabel, SystemLabel, SystemStage,
};

use crate::{
    Area, Attach, BackendStages, BackEndStartupStages, Color, Engen,
    FrontEndStages, FrontEndStartupStages, Job, Position,
};
use crate::extract::Extract;
use crate::gfx::{GfxSurface, GfxSurfaceConfiguration};
use crate::job::Container;
use crate::render::{Render, RenderPassHandle, RenderPhase};
use crate::text::atlas::AtlasBindGroup;
use crate::text::backend_system::{
    create_render_groups, render_group_differences, reset_extraction, resize_receiver,
};
use crate::text::coords::Coords;
use crate::text::extraction::Extraction;
use crate::text::frontend_system::{
    adjust_text_offset, bounds_diff, calc_area, calc_bound_from_guide, calc_scale_from_alignment,
    calc_text_offset_from_guide, depth_diff, discard_out_of_bounds, intercept_area_adjust,
    letter_diff, manage_render_groups, place, position_diff, pull_differences,
    setup as frontend_setup, visible_area_diff,
};
use crate::text::glyph::Key;
use crate::text::gpu_buffer::GpuBuffer;
use crate::text::index::Indexer;
use crate::text::null_bit::NullBit;
use crate::text::render_group::{DrawSection, RenderGroupBindGroup};
use crate::text::renderer::TextSystems::{CreateRenderGroups, RenderGroupDiff};
use crate::text::scale::AlignedFonts;
use crate::text::vertex::{GLYPH_AABB, Vertex, vertex_buffer};
use crate::viewport::Viewport;
use crate::window::ScaleFactor;

#[derive(Resource)]
pub struct TextRenderer {
    pub(crate) pipeline: wgpu::RenderPipeline,
    pub(crate) vertex_buffer: wgpu::Buffer,
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
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
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
            wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<Color>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Instance,
                attributes: &wgpu::vertex_attr_array![5 => Float32x4],
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
    let renderer = TextRenderer {
        pipeline,
        vertex_buffer: vertex_buffer(&gfx_surface),
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

#[derive(StageLabel)]
enum TextStages {
    PlacementPreparation,
    Placement,
    CalcArea,
    TextFrontEnd,
}

impl Attach for TextRenderer {
    fn attach(engen: &mut Engen) {
        engen
            .frontend
            .startup
            .add_system_to_stage(FrontEndStartupStages::Startup, frontend_setup);
        engen.frontend.main.add_stage_before(
            FrontEndStages::CoordAdjust,
            "area_intercept",
            SystemStage::single(intercept_area_adjust),
        );
        engen.frontend.main.add_stage_before(
            FrontEndStages::VisibilityPreparation,
            TextStages::PlacementPreparation,
            SystemStage::parallel()
                .with_system(calc_bound_from_guide)
                .with_system(calc_scale_from_alignment)
                .with_system(calc_text_offset_from_guide),
        );
        engen.frontend.main.add_stage_after(
            TextStages::PlacementPreparation,
            TextStages::Placement,
            SystemStage::parallel()
                .with_system(place)
                .with_system(adjust_text_offset),
        );
        engen.frontend.main.add_stage_after(
            TextStages::Placement,
            TextStages::CalcArea,
            SystemStage::single(calc_area),
        );
        engen.frontend.main.add_stage_after(
            FrontEndStages::ResolveVisibility,
            TextStages::TextFrontEnd,
            SystemStage::parallel(),
        );
        engen.frontend.main.add_system_to_stage(
            TextStages::TextFrontEnd,
            manage_render_groups.before("out of bounds"),
        );
        engen
            .frontend
            .main
            .add_system_to_stage(TextStages::TextFrontEnd, bounds_diff);
        engen
            .frontend
            .main
            .add_system_to_stage(TextStages::TextFrontEnd, depth_diff);
        engen
            .frontend
            .main
            .add_system_to_stage(TextStages::TextFrontEnd, position_diff);
        engen
            .frontend
            .main
            .add_system_to_stage(TextStages::TextFrontEnd, visible_area_diff);
        engen.frontend.main.add_system_to_stage(
            TextStages::TextFrontEnd,
            discard_out_of_bounds.label("out of bounds"),
        );
        engen.frontend.main.add_system_to_stage(
            TextStages::TextFrontEnd,
            letter_diff.label("letter diff").after("out of bounds"),
        );
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::Last, pull_differences);
        engen
            .backend
            .startup
            .add_system_to_stage(BackEndStartupStages::Setup, setup);
        engen.backend.main.add_system_to_stage(
            BackendStages::Prepare,
            create_render_groups.label(CreateRenderGroups),
        );
        engen.backend.main.add_system_to_stage(
            BackendStages::Prepare,
            render_group_differences
                .label(RenderGroupDiff)
                .after(CreateRenderGroups),
        );
        engen.backend.main.add_system_to_stage(
            BackendStages::Prepare,
            resize_receiver.after(RenderGroupDiff),
        );
        engen
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
                let glyph_areas = self
                    .container
                    .get::<GpuBuffer<Area>>(*render_group)
                    .expect("no glyph area buffer");
                render_pass_handle
                    .0
                    .set_vertex_buffer(4, glyph_areas.buffer.slice(..));
                let colors = self
                    .container
                    .get::<GpuBuffer<Color>>(*render_group)
                    .expect("no color buffer");
                render_pass_handle
                    .0
                    .set_vertex_buffer(5, colors.buffer.slice(..));
                let render_group_bind_group = self
                    .container
                    .get::<RenderGroupBindGroup>(*render_group)
                    .expect("no render group bind group");
                render_pass_handle
                    .0
                    .set_bind_group(2, &render_group_bind_group.bind_group, &[]);
                let atlas_bind_group = self
                    .container
                    .get::<AtlasBindGroup>(*render_group)
                    .expect("no atlas bind group");
                render_pass_handle
                    .0
                    .set_bind_group(3, &atlas_bind_group.bind_group, &[]);
                let draw_section = self.container.get::<DrawSection>(*render_group).expect("");
                if let Some(scissor_rect) = draw_section.section {
                    render_pass_handle.0.set_scissor_rect(
                        scissor_rect.position.x as u32,
                        scissor_rect.position.y as u32,
                        scissor_rect.area.width.max(1.0) as u32,
                        scissor_rect.area.height.max(1.0) as u32,
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
