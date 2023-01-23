use std::collections::HashMap;

use bevy_ecs::prelude::{Commands, Res, ResMut};
use wgpu::util::DeviceExt;

use crate::{Area, Canvas, Depth, Position};
use crate::text::coords::Coords;
use crate::text::extraction::Extraction;
use crate::text::font::MonoSpacedFont;
use crate::text::render_group::{NullBit, RenderGroup};
use crate::text::renderer::TextRenderer;
use crate::text::vertex::{GLYPH_AABB, Vertex};

fn sampler_resources(canvas: &Canvas) -> (wgpu::BindGroupLayout, wgpu::Sampler, wgpu::BindGroup) {
    let sampler_bind_group_layout_descriptor = wgpu::BindGroupLayoutDescriptor {
        label: Some("sampler bind group layout"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
            count: None,
        }],
    };
    let sampler_bind_group_layout = canvas
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
    let sampler = canvas.device.create_sampler(&sampler_descriptor);
    let sampler_bind_group_descriptor = wgpu::BindGroupDescriptor {
        label: Some("sampler bind group"),
        layout: &sampler_bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: wgpu::BindingResource::Sampler(&sampler),
        }],
    };
    let sampler_bind_group = canvas
        .device
        .create_bind_group(&sampler_bind_group_descriptor);
    (sampler_bind_group_layout, sampler, sampler_bind_group)
}

fn render_group_resources(canvas: &Canvas) -> wgpu::BindGroupLayout {
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
                visibility: wgpu::ShaderStages::VERTEX,
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
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 3,
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
    let render_group_bind_group_layout = canvas
        .device
        .create_bind_group_layout(&render_group_bind_group_layout_descriptor);
    render_group_bind_group_layout
}

fn pipeline(
    canvas: &Canvas,
    sampler_bind_group_layout: &wgpu::BindGroupLayout,
    render_group_bind_group_layout: &wgpu::BindGroupLayout,
) -> wgpu::RenderPipeline {
    let layout_descriptor = wgpu::PipelineLayoutDescriptor {
        label: Some("text pipeline layout descriptor"),
        bind_group_layouts: &[
            &canvas.viewport.bind_group_layout,
            &sampler_bind_group_layout,
            &render_group_bind_group_layout,
        ],
        push_constant_ranges: &[],
    };
    let layout = canvas.device.create_pipeline_layout(&layout_descriptor);
    let shader = canvas
        .device
        .create_shader_module(wgpu::include_wgsl!("text.wgsl"));
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
        format: canvas.viewport.depth_format,
        depth_write_enabled: true,
        depth_compare: wgpu::CompareFunction::Less,
        stencil: wgpu::StencilState::default(),
        bias: wgpu::DepthBiasState::default(),
    });
    let fragment_state = wgpu::FragmentState {
        module: &shader,
        entry_point: "fragment_entry",
        targets: &[Some(wgpu::ColorTargetState {
            format: canvas.surface_configuration.format,
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
    let pipeline = canvas.device.create_render_pipeline(&descriptor);
    pipeline
}

fn vertex_buffer(canvas: &Canvas) -> wgpu::Buffer {
    canvas
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("text vertex buffer"),
            contents: bytemuck::cast_slice(&GLYPH_AABB),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        })
}

pub(crate) fn setup(canvas: Res<Canvas>, mut cmd: Commands) {
    let (sampler_bind_group_layout, sampler, sampler_bind_group) = sampler_resources(&canvas);
    let render_group_bind_group_layout = render_group_resources(&canvas);
    let pipeline = pipeline(
        &canvas,
        &sampler_bind_group_layout,
        &render_group_bind_group_layout,
    );
    cmd.insert_resource(TextRenderer {
        pipeline,
        vertex_buffer: vertex_buffer(&canvas),
        render_groups: HashMap::new(),
        render_group_bind_group_layout,
        sampler,
        sampler_bind_group,
    });
    cmd.insert_resource(Extraction::new());
    cmd.insert_resource(MonoSpacedFont::default());
}

pub(crate) fn create_render_groups(
    mut extraction: ResMut<Extraction>,
    mut renderer: ResMut<TextRenderer>,
    canvas: Res<Canvas>,
) {
    for entity in extraction.removed_render_groups.iter() {
        renderer.render_groups.remove(entity);
    }
    for (entity, (max, position, depth, color, atlas_block, unique_glyphs)) in
    extraction.added_render_groups.iter()
    {
        let render_group = RenderGroup::new(
            &canvas,
            &renderer.render_group_bind_group_layout,
            *max as u32,
            *position,
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

pub(crate) fn render_group_differences(
    mut extraction: ResMut<Extraction>,
    mut renderer: ResMut<TextRenderer>,
    canvas: Res<Canvas>,
    font: Res<MonoSpacedFont>,
) {
    for (entity, difference) in extraction.differences.iter() {
        let render_group = renderer
            .render_groups
            .get_mut(entity)
            .expect(format!("no render group for {:?}", *entity).as_str());
        render_group.set_bounds(difference.bounds);
        if let Some(position) = difference.position {
            render_group.queue_position(position);
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

        render_group.write(&canvas);
    }
}
