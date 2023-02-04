use std::collections::HashMap;

use bevy_ecs::prelude::{Commands, EventReader, Res, ResMut};
use wgpu::util::DeviceExt;

use crate::coord::{Area, Position, ScaledPosition};
use crate::gfx::{GfxSurface, GfxSurfaceConfiguration};
use crate::text::coords::Coords;
use crate::text::extraction::Extraction;
use crate::text::font::MonoSpacedFont;
use crate::text::render_group::{NullBit, RenderGroup};
use crate::text::renderer::TextRenderer;
use crate::text::scale::TextScale;
use crate::text::vertex::{GLYPH_AABB, Vertex};
use crate::TextScaleAlignment;
use crate::viewport::Viewport;
use crate::window::{Resize, ScaleFactor};

fn sampler_resources(
    gfx_surface: &GfxSurface,
) -> (wgpu::BindGroupLayout, wgpu::Sampler, wgpu::BindGroup) {
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
    (sampler_bind_group_layout, sampler, sampler_bind_group)
}

fn render_group_resources(gfx_surface: &GfxSurface) -> wgpu::BindGroupLayout {
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
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
        ],
    };
    let render_group_bind_group_layout = gfx_surface
        .device
        .create_bind_group_layout(&render_group_bind_group_layout_descriptor);
    render_group_bind_group_layout
}

fn pipeline(
    gfx_surface: &GfxSurface,
    gfx_surface_config: &GfxSurfaceConfiguration,
    viewport: &Viewport,
    sampler_bind_group_layout: &wgpu::BindGroupLayout,
    render_group_bind_group_layout: &wgpu::BindGroupLayout,
) -> wgpu::RenderPipeline {
    let layout_descriptor = wgpu::PipelineLayoutDescriptor {
        label: Some("text pipeline layout descriptor"),
        bind_group_layouts: &[
            &viewport.bind_group_layout,
            &sampler_bind_group_layout,
            &render_group_bind_group_layout,
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
    pipeline
}

fn vertex_buffer(gfx_surface: &GfxSurface) -> wgpu::Buffer {
    gfx_surface
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("text vertex buffer"),
            contents: bytemuck::cast_slice(&GLYPH_AABB),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        })
}

pub(crate) fn setup(
    gfx_surface: Res<GfxSurface>,
    gfx_surface_config: Res<GfxSurfaceConfiguration>,
    viewport: Res<Viewport>,
    scale_factor: Res<ScaleFactor>,
    mut cmd: Commands,
) {
    let (sampler_bind_group_layout, sampler, sampler_bind_group) = sampler_resources(&gfx_surface);
    let render_group_bind_group_layout = render_group_resources(&gfx_surface);
    let pipeline = pipeline(
        &gfx_surface,
        &gfx_surface_config,
        &viewport,
        &sampler_bind_group_layout,
        &render_group_bind_group_layout,
    );
    cmd.insert_resource(TextRenderer {
        pipeline,
        vertex_buffer: vertex_buffer(&gfx_surface),
        render_groups: HashMap::new(),
        render_group_bind_group_layout,
        sampler,
        sampler_bind_group,
    });
    cmd.insert_resource(Extraction::new());
    cmd.insert_resource(MonoSpacedFont::jet_brains_mono(
        TextScale::from_alignment(TextScaleAlignment::Medium, scale_factor.factor).scale,
    ));
}

pub(crate) fn create_render_groups(
    mut extraction: ResMut<Extraction>,
    mut renderer: ResMut<TextRenderer>,
    gfx_surface: Res<GfxSurface>,
    scale_factor: Res<ScaleFactor>,
) {
    for entity in extraction.removed_render_groups.iter() {
        renderer.render_groups.remove(entity);
    }
    for (entity, (max, position, visible_section, depth, color, atlas_block, unique_glyphs)) in
    extraction.added_render_groups.iter()
    {
        let render_group = RenderGroup::new(
            &gfx_surface,
            &renderer.render_group_bind_group_layout,
            *max as u32,
            position.to_scaled(scale_factor.factor),
            *visible_section,
            *depth,
            *color,
            *atlas_block,
            *unique_glyphs as u32,
        );
        renderer.render_groups.insert(*entity, render_group);
    }
}

pub(crate) fn reset_extraction(mut extraction: ResMut<Extraction>) {
    *extraction = Extraction::new();
}

pub(crate) fn resize_receiver(
    mut renderer: ResMut<TextRenderer>,
    mut event_reader: EventReader<Resize>,
    viewport: Res<Viewport>,
) {
    for event in event_reader.iter() {
        for (_entity, mut group) in renderer.render_groups.iter_mut() {
            group.adjust_draw_section(viewport.as_section(), event.scale_factor);
        }
    }
}

pub(crate) fn render_group_differences(
    mut extraction: ResMut<Extraction>,
    mut renderer: ResMut<TextRenderer>,
    gfx_surface: Res<GfxSurface>,
    font: Res<MonoSpacedFont>,
    viewport: Res<Viewport>,
    scale_factor: Res<ScaleFactor>,
) {
    for (entity, difference) in extraction.differences.iter() {
        let render_group = renderer
            .render_groups
            .get_mut(entity)
            .expect(format!("no render group for {:?}", *entity).as_str());
        let mut draw_section_need_resize = false;
        if let Some(bounds) = difference.bounds {
            render_group.bound_section.replace(bounds);
            draw_section_need_resize = true;
        }
        if let Some(visible_section) = difference.visible_section {
            render_group.visible_section = visible_section;
            draw_section_need_resize = true;
        }
        if draw_section_need_resize {
            render_group.adjust_draw_section(viewport.as_section(), scale_factor.factor);
        }
        if let Some(position) = difference.position {
            render_group.queue_position(position.to_scaled(scale_factor.factor));
        }
        if let Some(depth) = difference.depth {
            render_group.queue_depth(depth);
        }
        if let Some(color) = difference.color {
            render_group.queue_color(color);
        }
        for key in difference.remove.iter() {
            render_group.remove(*key);
        }
        for (key, glyph_position) in difference.add.iter() {
            render_group.add(*key, *glyph_position);
        }
        for (key, glyph_position) in difference.update.iter() {
            render_group.queue_glyph_position(*key, *glyph_position);
        }
        for glyph_id in difference.glyph_remove.iter() {
            render_group.remove_glyph(*glyph_id);
        }
        for (key, glyph) in difference.glyph_add.iter() {
            render_group.add_glyph(*key, glyph.clone(), &font);
            let (coords, glyph_area) = render_group.read_glyph_info(*key);
            render_group.queue_glyph_info(*key, coords, glyph_area);
        }
        render_group.write(&gfx_surface);
    }
}
