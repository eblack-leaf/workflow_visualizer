use bevy_ecs::event::EventWriter;
use std::collections::HashMap;

use bevy_ecs::prelude::{
    Commands, Component, Entity, Event, NonSend, NonSendMut, Query, Res, ResMut, Resource,
};
use compact_str::CompactString;
use image::{EncodableLayout, GenericImageView};
use serde::{Deserialize, Serialize};
use wgpu::util::DeviceExt;

use crate::images::render_group::ImageRenderGroup;
use crate::orientation::{AspectRatio, Orientation};
use crate::texture_atlas::{AtlasLocation, TextureSampler};
use crate::uniform::vertex_bind_group_layout_entry;
use crate::{
    Area, AtlasBlock, AtlasDimension, AtlasTextureDimensions, GfxSurface, GfxSurfaceConfiguration,
    MsaaRenderAdapter, NumericalContext, RawPosition, Render, RenderPassHandle, RenderPhase,
    ScaleFactor, TextureAtlas, TextureBindGroup, TextureCoordinates, Viewport, Visualizer,
};

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
            label: Some("images vertex buffer"),
            contents: bytemuck::cast_slice(&AABB),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        })
}
#[derive(Component, Copy, Clone, PartialOrd, PartialEq, Default, Debug)]
pub struct ImageFade(pub f32);
#[derive(
    Component, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, Debug,
)]
pub struct ImageHandle(pub i32);
impl From<i32> for ImageHandle {
    fn from(value: i32) -> Self {
        ImageHandle(value)
    }
}
#[derive(Component, Clone, Serialize, Deserialize, Debug)]
pub struct ImageRequest {
    pub handle: ImageHandle,
    pub data: Vec<u8>,
}
impl ImageRequest {
    pub fn new<IN: Into<ImageHandle>, D: Into<Vec<u8>>>(handle: IN, data: D) -> Self {
        Self {
            handle: handle.into(),
            data: data.into(),
        }
    }
}

pub(crate) struct ImageData {
    pub(crate) atlas: TextureAtlas,
    pub(crate) bind_group: TextureBindGroup,
    pub(crate) coordinates: TextureCoordinates,
}
impl ImageData {
    pub(crate) fn new(
        atlas: TextureAtlas,
        bind_group: TextureBindGroup,
        coordinates: TextureCoordinates,
    ) -> Self {
        Self {
            atlas,
            bind_group,
            coordinates,
        }
    }
}

#[cfg_attr(not(target_family = "wasm"), derive(Resource))]
pub(crate) struct ImageRenderer {
    pub(crate) pipeline: wgpu::RenderPipeline,
    pub(crate) render_groups: HashMap<Entity, ImageRenderGroup>,
    pub(crate) vertex_buffer: wgpu::Buffer,
    pub(crate) sampler_bind_group: wgpu::BindGroup,
    pub(crate) images: HashMap<ImageHandle, ImageData>,
    pub(crate) render_group_layout: wgpu::BindGroupLayout,
    pub(crate) render_group_uniforms_layout: wgpu::BindGroupLayout,
}
#[derive(Resource, Default)]
pub struct ImageOrientations(pub(crate) HashMap<ImageHandle, Orientation>);
impl ImageOrientations {
    pub fn get<IN: Into<ImageHandle>>(&self, name: IN) -> Orientation {
        self.0.get(&name.into()).copied().expect("orientation")
    }
}
#[derive(Resource, Default)]
pub struct ImageSizes(pub(crate) HashMap<ImageHandle, Area<NumericalContext>>);
impl ImageSizes {
    pub fn get<IN: Into<ImageHandle>>(&self, name: IN) -> Area<NumericalContext> {
        self.0.get(&name.into()).copied().expect("size")
    }
}
#[derive(Event, Copy, Clone)]
pub struct ImageLoaded(pub ImageHandle);
pub(crate) fn load_images(
    #[cfg(not(target_family = "wasm"))] mut image_renderer: ResMut<ImageRenderer>,
    #[cfg(target_family = "wasm")] mut image_renderer: NonSendMut<ImageRenderer>,
    requests: Query<(Entity, &ImageRequest)>,
    mut cmd: Commands,
    #[cfg(not(target_family = "wasm"))] gfx: Res<GfxSurface>,
    #[cfg(target_family = "wasm")] gfx: NonSend<GfxSurface>,
    mut orientations: ResMut<ImageOrientations>,
    mut sizes: ResMut<ImageSizes>,
    mut event_writer: EventWriter<ImageLoaded>,
) {
    for (entity, request) in requests.iter() {
        let image = image::load_from_memory(request.data.as_slice()).expect("images-load");
        let texture_data = image.to_rgba8();
        let dimensions = image.dimensions();
        sizes.0.insert(request.handle, Area::from(dimensions));
        let block = AtlasBlock::new((dimensions.0, dimensions.1));
        let atlas_dimension = AtlasDimension::new(1);
        let dimensions = AtlasTextureDimensions::new(block, atlas_dimension);
        let atlas = TextureAtlas::new(
            &gfx,
            block,
            atlas_dimension,
            wgpu::TextureFormat::Rgba8UnormSrgb,
        );
        let coordinates = atlas.write::<[u8; 4]>(
            AtlasLocation::new(0, 0),
            texture_data.as_bytes(),
            block.block,
            &gfx,
        );
        let bind_group =
            TextureBindGroup::new(&gfx, &image_renderer.render_group_layout, atlas.view());
        orientations
            .0
            .insert(request.handle, Orientation::new(dimensions.dimensions));
        image_renderer.images.insert(
            request.handle,
            ImageData::new(atlas, bind_group, coordinates),
        );
        event_writer.send(ImageLoaded(request.handle));
        cmd.entity(entity).despawn();
    }
}

impl Render for ImageRenderer {
    fn setup(
        _visualizer: &Visualizer,
        gfx: &GfxSurface,
        viewport: &Viewport,
        gfx_config: &GfxSurfaceConfiguration,
        msaa: &MsaaRenderAdapter,
        scale_factor: &ScaleFactor,
    ) -> Self {
        let sampler = TextureSampler::new(&gfx);
        let sampler_bind_group_layout_descriptor = wgpu::BindGroupLayoutDescriptor {
            label: Some("sampler bind group layout"),
            entries: &[TextureSampler::layout_entry(0)],
        };
        let sampler_bind_group_layout = gfx
            .device
            .create_bind_group_layout(&sampler_bind_group_layout_descriptor);
        let sampler_bind_group_descriptor = wgpu::BindGroupDescriptor {
            label: Some("sampler bind group"),
            layout: &sampler_bind_group_layout,
            entries: &[TextureSampler::bind_group_entry(&sampler.sampler, 0)],
        };
        let sampler_bind_group = gfx.device.create_bind_group(&sampler_bind_group_descriptor);
        let texture_bind_group_layout =
            gfx.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("images-render-group-layout"),
                    entries: &[TextureBindGroup::entry(0)],
                });
        let render_group_uniforms_layout =
            gfx.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("render-group"),
                    entries: &[
                        vertex_bind_group_layout_entry(0),
                        vertex_bind_group_layout_entry(1),
                        vertex_bind_group_layout_entry(2),
                    ],
                });
        let pipeline_layout_descriptor = wgpu::PipelineLayoutDescriptor {
            label: Some("images-render-pipeline-layout"),
            bind_group_layouts: &[
                viewport.bind_group_layout(),
                &sampler_bind_group_layout,
                &texture_bind_group_layout,
                &render_group_uniforms_layout,
            ],
            push_constant_ranges: &[],
        };
        let pipeline_layout = gfx
            .device
            .create_pipeline_layout(&pipeline_layout_descriptor);
        let shader = gfx
            .device
            .create_shader_module(wgpu::include_wgsl!("images.wgsl"));
        let fragment_targets = [Some(gfx_config.alpha_color_target_state())];
        let pipeline_descriptor = wgpu::RenderPipelineDescriptor {
            label: Some("images renderer"),
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
            primitive: gfx.triangle_primitive(),
            depth_stencil: Some(viewport.depth_stencil_state()),
            multisample: msaa.multisample_state(),
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fragment_entry",
                targets: &fragment_targets,
            }),
            multiview: None,
        };
        let pipeline = gfx.device.create_render_pipeline(&pipeline_descriptor);
        let renderer = ImageRenderer {
            pipeline,
            render_groups: HashMap::new(),
            vertex_buffer: aabb_vertex_buffer(&gfx),
            sampler_bind_group,
            images: HashMap::new(),
            render_group_layout: texture_bind_group_layout,
            render_group_uniforms_layout,
        };
        renderer
    }

    fn phase() -> RenderPhase {
        RenderPhase::Alpha(7)
    }

    fn render<'a>(&'a self, render_pass_handle: &mut RenderPassHandle<'a>, viewport: &'a Viewport) {
        render_pass_handle.0.set_pipeline(&self.pipeline);
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
            render_pass_handle.0.set_bind_group(
                2,
                &self
                    .images
                    .get(&group.image_name)
                    .expect("no images")
                    .bind_group
                    .bind_group,
                &[],
            );
            render_pass_handle
                .0
                .set_bind_group(3, &group.render_group_bind_group, &[]);
            render_pass_handle.0.draw(0..AABB.len() as u32, 0..1);
        }
    }
}
