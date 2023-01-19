mod font;
mod index;

use crate::canvas::{Viewport, Visibility};
use crate::r_text::index::{Index, Indexer};
use crate::render::{Render, RenderPassHandle, RenderPhase};
use crate::task::Stage;
use crate::{Area, Attach, Canvas, Color, Depth, Engen, Position, Section, Task};
use bevy_ecs::prelude::{
    Added, Bundle, Changed, Commands, Component, Entity, Or, Query, RemovedComponents, Res, ResMut,
    Resource, With, *,
};
pub(crate) use font::Font;
use std::collections::{HashMap, HashSet};
use std::num::NonZeroU32;
use wgpu::util::DeviceExt;
use wgpu::{
    include_wgsl, vertex_attr_array, BufferAddress, Extent3d, ImageCopyTexture, ImageDataLayout,
    Origin3d, SamplerBindingType, TextureAspect, TextureDimension, TextureFormat,
    TextureSampleType, TextureUsages, TextureViewDescriptor, TextureViewDimension, VertexAttribute,
    VertexState,
};

#[derive(Bundle)]
pub struct TextBundle {
    pub text: Text,
    pub scale: Scale,
    pub position: Position,
    pub depth: Depth,
    pub color: Color,
    // auto made
    pub(crate) placer: Placer,
    pub(crate) keys: Keys,
}
#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub(crate) struct TextOffset(pub usize);
#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub(crate) struct Key {
    pub entity: Entity,
    pub offset: TextOffset,
}
impl Key {
    pub(crate) fn new(entity: Entity, offset: TextOffset) -> Self {
        Self { entity, offset }
    }
}
#[derive(Component)]
pub struct Text {
    pub string: String,
}
#[derive(Component, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Scale {
    pub scale: u32,
}
impl Scale {
    pub fn new(scale: u32) -> Self {
        Self { scale }
    }
    pub fn px(&self) -> f32 {
        self.scale as f32
    }
}
impl From<f32> for Scale {
    fn from(scale: f32) -> Self {
        Self {
            scale: scale as u32,
        }
    }
}
impl From<u32> for Scale {
    fn from(scale: u32) -> Self {
        Self { scale }
    }
}
#[derive(Component)]
pub struct Placer {
    pub placer: fontdue::layout::Layout,
}
#[derive(Component)]
pub(crate) struct Keys {
    pub keys: HashSet<Key>,
}
pub(crate) struct Alignment {
    pub(crate) dimensions: [f32; 2],
}
impl Alignment {
    pub(crate) fn new(dimensions: [f32; 2]) -> Self {
        Self { dimensions }
    }
}
#[derive(Hash, Eq, PartialEq, Clone)]
pub(crate) struct Glyph {
    pub(crate) hash: GlyphHash,
    pub(crate) character: char,
    pub(crate) scale: Scale,
}
impl Glyph {
    pub(crate) fn new(hash: GlyphHash, character: char, scale: Scale) -> Self {
        Self {
            hash,
            character,
            scale,
        }
    }
}
#[derive(Resource)]
pub(crate) struct Changes {
    pub added_text_entities: HashMap<Entity, (u32, Alignment)>,
    pub removed_text_entities: HashSet<Entity>,
    pub adds: HashMap<Entity, HashMap<Key, (Area, Attributes)>>,
    pub updates: HashMap<Entity, HashMap<Key, Attributes>>,
    pub removes: HashMap<Entity, HashSet<Key>>,
    pub glyphs: HashMap<Entity, HashSet<(Key, Glyph)>>,
    pub bounds: HashMap<Entity, Section>,
    pub removed_bounds: HashSet<Entity>,
    pub removed_glyphs: HashMap<Entity, HashSet<(Key, GlyphHash)>>,
    pub visibility: HashMap<Entity, Visibility>,
}
impl Changes {
    pub(crate) fn reset(&mut self) {
        self.added_text_entities.clear();
        self.removed_text_entities.clear();
        self.adds.clear();
        self.updates.clear();
        self.removes.clear();
        self.glyphs.clear();
        self.bounds.clear();
        self.removed_bounds.clear();
        self.removed_glyphs.clear();
        self.visibility.clear();
    }
    pub(crate) fn new() -> Self {
        Self {
            added_text_entities: HashMap::new(),
            removed_text_entities: HashSet::new(),
            adds: HashMap::new(),
            updates: HashMap::new(),
            removes: HashMap::new(),
            glyphs: HashMap::new(),
            bounds: HashMap::new(),
            removed_bounds: HashSet::new(),
            removed_glyphs: HashMap::new(),
            visibility: HashMap::new(),
        }
    }
}
pub(crate) type GlyphHash = fontdue::layout::GlyphRasterConfig;
#[derive(Resource)]
pub(crate) struct Cache {
    pub glyphs: HashMap<Key, GlyphHash>,
    pub attributes: HashMap<Key, Attributes>,
    pub bounds: HashMap<Entity, Section>,
    pub visible_entities: HashSet<Entity>,
}
impl Cache {
    pub(crate) fn new() -> Self {
        Self {
            glyphs: HashMap::new(),
            attributes: HashMap::new(),
            bounds: HashMap::new(),
            visible_entities: HashSet::new(),
        }
    }
}
pub(crate) struct InstanceBuffer {
    pub(crate) cpu: Vec<Instance>,
    pub(crate) gpu: wgpu::Buffer,
    pub(crate) indexer: Indexer<Key>,
}
impl InstanceBuffer {
    pub(crate) fn add(&mut self, key: Key, instance: Instance) {}
    pub(crate) fn write(&mut self, canvas: &Canvas) {}
    pub(crate) fn update(&mut self, key: Key, attributes: Attributes) {}
    pub(crate) fn remove(&mut self, key: Key) {
        let removed_index = self.indexer.remove(&key);
        if let Some(index) = removed_index {
            self.queue_write(index, Instance::nullified());
        }
    }
    pub(crate) fn queue_write(&mut self, index: Index, instance: Instance) {
        todo!()
    }
    pub(crate) fn new(canvas: &Canvas, initial_supported_instances: u32) -> Self {
        Self {
            cpu: Vec::new(),
            gpu: canvas.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("text instance buffer"),
                size: (std::mem::size_of::<Instance>() * initial_supported_instances as usize)
                    as BufferAddress,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }),
            indexer: Indexer::new(initial_supported_instances),
        }
    }
    pub(crate) fn count(&self) -> usize {
        self.cpu.len()
    }
}
const GLYPH_AABB: [Vertex; 6] = [
    Vertex::new(Position::new(0.0, 0.0)),
    Vertex::new(Position::new(0.0, 1.0)),
    Vertex::new(Position::new(1.0, 0.0)),
    Vertex::new(Position::new(1.0, 0.0)),
    Vertex::new(Position::new(0.0, 1.0)),
    Vertex::new(Position::new(1.0, 1.0)),
];
#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Copy, Clone)]
pub(crate) struct Vertex {
    pub position: Position,
}
impl Vertex {
    pub const fn new(position: Position) -> Self {
        Self { position }
    }
}
#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Copy, Clone)]
pub struct Attributes {
    pub position: Position,
    pub depth: Depth,
    pub color: Color,
}
impl Attributes {
    pub fn new(position: Position, depth: Depth, color: Color) -> Self {
        Self {
            position,
            depth,
            color,
        }
    }
}
#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Copy, Clone)]
pub struct TexCoords {
    pub coords: [f32; 4],
}
impl TexCoords {
    pub fn new(coords: [f32; 4]) -> Self {
        Self { coords }
    }
}
#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Copy, Clone)]
pub struct Instance {
    pub attributes: Attributes,
    pub area: Area,
    pub tex_coords: TexCoords,
}
impl Instance {
    pub fn new(attributes: Attributes, area: Area, tex_coords: TexCoords) -> Self {
        Self {
            attributes,
            area,
            tex_coords,
        }
    }
    pub fn nullified() -> Self {
        Self {
            attributes: Attributes::new(Position::default(), Depth::default(), Color::default()),
            area: Area::default(),
            tex_coords: TexCoords::new([0.0, 0.0, 0.0, 0.0]),
        }
    }
}
#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub(crate) struct GridLocation {
    pub(crate) x: u32,
    pub(crate) y: u32,
}
impl GridLocation {
    pub(crate) fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}
pub(crate) type Bitmap = Vec<u8>;
#[derive(Clone)]
pub(crate) struct GridEntry {
    pub(crate) bitmap: Bitmap,
    pub(crate) area: Area,
}
impl GridEntry {
    pub(crate) fn new(bitmap: Bitmap, area: Area) -> Self {
        Self { bitmap, area }
    }
}
pub(crate) struct GlyphReference {
    pub(crate) ref_count: u32,
}
impl GlyphReference {
    pub(crate) fn new() -> Self {
        Self { ref_count: 0 }
    }
    pub(crate) fn increment(&mut self) {
        self.ref_count += 1;
    }
    pub(crate) fn decrement(&mut self) {
        let sub_value = 1 * (self.ref_count == 0) as u32;
        self.ref_count -= sub_value;
    }
}
pub(crate) struct Rasterization {
    pub(crate) instances: InstanceBuffer,
    pub(crate) texture: wgpu::Texture,
    pub(crate) view: wgpu::TextureView,
    pub(crate) alignment: Alignment,
    pub(crate) logical_dimension: u32,
    pub(crate) next_location: Option<GridLocation>,
    pub(crate) bind_group: wgpu::BindGroup,
    pub(crate) bounds: Option<Section>,
    pub(crate) grid: HashMap<GridLocation, GridEntry>,
    pub(crate) glyph_locations: HashMap<GlyphHash, GridLocation>,
    pub(crate) tex_coords: HashMap<GlyphHash, TexCoords>,
    pub(crate) references: HashMap<GlyphHash, GlyphReference>,
    pub(crate) free_blocks: HashSet<GridLocation>,
    pub(crate) keyed_glyph_hashes: HashMap<Key, GlyphHash>,
}
pub(crate) fn rasterize(
    font: Res<Font>,
    mut renderer: ResMut<Renderer>,
    extraction: Res<Extraction>,
    canvas: Res<Canvas>,
) {
    for (entity, set) in extraction.changes.removed_glyphs.iter() {
        for (key, glyph_hash) in set.iter() {
            renderer
                .rasterizations
                .get_mut(&entity)
                .expect("no rasterization for entity")
                .references
                .get_mut(glyph_hash)
                .expect("no references")
                .decrement();
            renderer
                .rasterizations
                .get_mut(&entity)
                .expect("no rasterization for entity")
                .keyed_glyph_hashes
                .remove(key);
        }
    }
    for (entity, glyphs) in extraction.changes.glyphs.iter() {
        for (key, glyph) in glyphs.iter() {
            renderer
                .rasterizations
                .get_mut(&entity)
                .expect("no rasterization for entity")
                .keyed_glyph_hashes
                .insert(*key, glyph.hash);
            if renderer
                .rasterizations
                .get(&entity)
                .expect("no rasterization")
                .glyph_locations
                .contains_key(&glyph.hash)
            {
                renderer
                    .rasterizations
                    .get_mut(&entity)
                    .expect("")
                    .references
                    .get_mut(&glyph.hash)
                    .expect("no references")
                    .increment();
            } else {
                let (metrics, bitmap) = font.font().rasterize(glyph.character, glyph.scale.px());
                renderer
                    .rasterizations
                    .get_mut(&entity)
                    .expect("no rasterization for entity")
                    .add(
                        &canvas,
                        GridEntry::new(bitmap, (metrics.width, metrics.height).into()),
                        glyph.hash,
                    );
                renderer
                    .rasterizations
                    .get_mut(&entity)
                    .expect("")
                    .references
                    .insert(glyph.hash, GlyphReference::new());
            }
        }
    }
}
impl Rasterization {
    fn total_x(&self) -> f32 {
        self.alignment.dimensions[0] * self.logical_dimension as f32
    }
    fn total_y(&self) -> f32 {
        self.alignment.dimensions[1] * self.logical_dimension as f32
    }
    fn place(
        &mut self,
        canvas: &Canvas,
        placement_location: GridLocation,
        grid_entry: GridEntry,
        glyph_hash: GlyphHash,
    ) {
        let position = self.position(placement_location);
        let normalized_position: Position =
            (position.x / self.total_x(), position.y / self.total_y()).into();
        let normalized_area: Area = (
            grid_entry.area.width / self.total_x(),
            grid_entry.area.height / self.total_y(),
        )
            .into();
        self.grid.insert(placement_location, grid_entry.clone());
        let image_copy_texture = ImageCopyTexture {
            texture: &self.texture,
            mip_level: 1,
            // should use normalized position to be in texels?
            origin: Origin3d {
                x: position.x as u32,
                y: position.y as u32,
                z: 0,
            },
            aspect: TextureAspect::All,
        };
        let image_data_layout = ImageDataLayout {
            offset: 1,
            bytes_per_row: NonZeroU32::new(grid_entry.area.width as u32),
            rows_per_image: NonZeroU32::new(grid_entry.area.height as u32),
        };
        let extent = Extent3d {
            width: grid_entry.area.width as u32,
            height: grid_entry.area.height as u32,
            depth_or_array_layers: 1,
        };
        canvas.queue.write_texture(
            image_copy_texture,
            bytemuck::cast_slice(&grid_entry.bitmap),
            image_data_layout,
            extent,
        );
        let tex_coords = TexCoords::new([
            normalized_position.x,
            normalized_position.y,
            normalized_position.x + normalized_area.width,
            normalized_position.y + normalized_area.height,
        ]);
        self.tex_coords.insert(glyph_hash, tex_coords);
        let maybe_next = match placement_location.x + 1 == self.logical_dimension {
            true => {
                let mut value = None;
                if !placement_location.y == self.logical_dimension - 1 {
                    let x = 0;
                    let y = placement_location.y + 1;
                    value = Some(GridLocation::new(x, y))
                }
                value
            }
            false => {
                let x = placement_location.x + 1;
                let y = placement_location.y;
                Some(GridLocation::new(x, y))
            }
        };
        if let Some(next) = maybe_next {
            self.next_location.replace(next);
        }
    }
    pub(crate) fn add(&mut self, canvas: &Canvas, grid_entry: GridEntry, glyph_hash: GlyphHash) {
        let next_location = self.next_location.take();
        if let Some(placement_location) = next_location {
            self.place(canvas, placement_location, grid_entry, glyph_hash);
        } else {
            if self.free_blocks.is_empty() {
                panic!("pepbutt is eating the tree")
            } else {
                let location = *self.free_blocks.iter().next().take().unwrap();
                self.place(canvas, location, grid_entry, glyph_hash);
            }
        }
    }
    fn position(&self, grid_location: GridLocation) -> Position {
        (
            grid_location.x as f32 * self.alignment.dimensions[0],
            grid_location.y as f32 * self.alignment.dimensions[1],
        )
            .into()
    }
    pub(crate) fn new(
        canvas: &Canvas,
        bind_group_layout: &wgpu::BindGroupLayout,
        alignment: Alignment,
        initial_supported_instances: u32,
    ) -> Self {
        let instances = InstanceBuffer::new(canvas, initial_supported_instances);
        let mut logical_dimension = (initial_supported_instances as f32).sqrt() as u32;
        while logical_dimension.pow(2) < initial_supported_instances {
            logical_dimension += 1;
        }
        let texture_width: u32 = logical_dimension * alignment.dimensions[0] as u32;
        let texture_height: u32 = logical_dimension * alignment.dimensions[1] as u32;
        let hardware_max = canvas.options.limits.max_texture_dimension_2d;
        if texture_width > hardware_max {
            panic!("requested larger than possible texture")
        }
        if texture_height > hardware_max {
            panic!("requested larger than possible texture")
        }
        let texture_descriptor = wgpu::TextureDescriptor {
            label: Some("texture atlas"),
            size: Extent3d {
                width: texture_width,
                height: texture_height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::R8Uint,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
        };
        let texture = canvas.device.create_texture(&texture_descriptor);
        let view = texture.create_view(&TextureViewDescriptor::default());
        let descriptor = wgpu::BindGroupDescriptor {
            label: Some("texture atlas bind group"),
            layout: bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&view),
            }],
        };
        let bind_group = canvas.device.create_bind_group(&descriptor);
        Self {
            instances,
            texture,
            view,
            alignment,
            logical_dimension,
            grid: HashMap::new(),
            glyph_locations: HashMap::new(),
            bind_group,
            bounds: None,
            next_location: Some(GridLocation::new(0, 0)),
            tex_coords: HashMap::new(),
            references: HashMap::new(),
            free_blocks: HashSet::new(),
            keyed_glyph_hashes: HashMap::new(),
        }
    }
}
#[derive(Resource)]
pub struct Renderer {
    pub(crate) pipeline: wgpu::RenderPipeline,
    pub(crate) vertex_buffer: wgpu::Buffer,
    pub(crate) rasterizations: HashMap<Entity, Rasterization>,
    pub(crate) visible_text_entities: HashSet<Entity>,
    pub(crate) rasterization_bind_group_layout: wgpu::BindGroupLayout,
    pub(crate) sampler: wgpu::Sampler,
    pub(crate) sampler_bind_group: wgpu::BindGroup,
}
pub fn render_setup(canvas: Res<Canvas>, mut cmd: Commands) {
    let sampler_bind_group_layout_descriptor = wgpu::BindGroupLayoutDescriptor {
        label: Some("sampler bind group layout"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Sampler(SamplerBindingType::Filtering),
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
        lod_min_clamp: 0.0,
        lod_max_clamp: 0.0,
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
    let rasterization_bind_group_layout_descriptor = wgpu::BindGroupLayoutDescriptor {
        label: Some("rasterization bind group"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Texture {
                sample_type: TextureSampleType::Uint,
                view_dimension: TextureViewDimension::D2,
                multisampled: false,
            },
            count: None,
        }],
    };
    let rasterization_bind_group_layout = canvas
        .device
        .create_bind_group_layout(&rasterization_bind_group_layout_descriptor);
    let layout_descriptor = wgpu::PipelineLayoutDescriptor {
        label: Some("text pipeline layout descriptor"),
        bind_group_layouts: &[
            &canvas.viewport.bind_group_layout,
            &sampler_bind_group_layout,
            &rasterization_bind_group_layout,
        ],
        push_constant_ranges: &[],
    };
    let layout = canvas.device.create_pipeline_layout(&layout_descriptor);
    let shader = canvas
        .device
        .create_shader_module(include_wgsl!("text.wgsl"));
    let vertex_state = VertexState {
        module: &shader,
        entry_point: "vertex_entry",
        buffers: &[
            wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &vertex_attr_array![0 => Float32x2],
            },
            wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<Instance>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Instance,
                attributes: &vertex_attr_array![
                    1 => Float32x2,
                    3 => Float32,
                    4 => Float32x4,
                    2 => Float32x2,
                    5 => Float32x4,
                ],
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
    let vertex_buffer = canvas
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("text vertex buffer"),
            contents: bytemuck::cast_slice(&GLYPH_AABB),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });
    let rasterizations = HashMap::new();
    cmd.insert_resource(Renderer {
        pipeline,
        vertex_buffer,
        rasterizations,
        rasterization_bind_group_layout,
        visible_text_entities: HashSet::new(),
        sampler,
        sampler_bind_group,
    });
    cmd.insert_resource(Extraction::new());
    cmd.insert_resource(Font::default());
}
pub(crate) fn compute_setup(mut cmd: Commands) {
    cmd.insert_resource(Changes::new());
    cmd.insert_resource(Cache::new());
    cmd.insert_resource(Font::default());
}
pub(crate) fn update_attrs(
    text: Query<
        (Entity, &Text, &Position, &Color, &Depth),
        (Or<(Changed<Position>, Changed<Color>, Changed<Depth>)>),
    >,
) {
}
pub(crate) fn emit(
    mut cache: ResMut<Cache>,
    mut changes: ResMut<Changes>,
    mut text: Query<
        (
            Entity,
            &Text,
            &mut Placer,
            &mut Keys,
            &Position,
            Option<&Area>,
            &Depth,
            &Color,
            &Scale,
            &Visibility,
        ),
        // if changed position or color or depth try to just write to changes.updates if present
        (Or<(Changed<Text>, Changed<Area>)>),
    >,
    font: Res<Font>,
) {
    for (
        entity,
        text,
        mut placer,
        mut keys,
        position,
        maybe_area,
        depth,
        color,
        scale,
        visibility,
    ) in text.iter_mut()
    {
        if visibility.visible {
            placer.placer.clear();
            placer.placer.append(
                font.font_slice(),
                &fontdue::layout::TextStyle::new(text.string.as_str(), scale.px(), Font::index()),
            );
            let mut retained_keys = HashSet::new();
            let mut added_keys = HashSet::new();
            if let Some(area) = maybe_area {
                cache.bounds.insert(entity, (*position, *area).into());
                changes.bounds.insert(entity, (*position, *area).into());
            } else {
                let old_bound = cache.bounds.remove(&entity);
                if old_bound.is_some() {
                    changes.removed_bounds.insert(entity);
                }
            }
            let mut glyphs = HashSet::new();
            let mut removed_glyphs = HashSet::new();
            let mut removes = HashSet::new();
            let mut adds = HashMap::new();
            for positioned_glyph in placer.placer.glyphs() {
                let hash = positioned_glyph.key;
                let glyph = Glyph::new(hash, positioned_glyph.parent, *scale);
                let key = Key::new(entity, TextOffset(positioned_glyph.byte_offset));
                let glyph_section = Section::new(
                    *position + (positioned_glyph.x, positioned_glyph.y).into(),
                    (positioned_glyph.width, positioned_glyph.height).into(),
                );
                if let Some(area) = maybe_area {
                    let text_section = Section::new(*position, *area);
                    if !(text_section.left() < glyph_section.right()
                        && text_section.right() > glyph_section.left()
                        && text_section.top() < glyph_section.bottom()
                        && text_section.bottom() > glyph_section.top())
                    {
                        if cache.attributes.contains_key(&key) {
                            removes.insert(key);
                        }
                        continue;
                    }
                }
                let current_attributes = Attributes::new(glyph_section.position, *depth, *color);
                if cache.attributes.contains_key(&key) {
                    retained_keys.insert(key);
                    let cached_glyph = cache.glyphs.get(&key).expect("no cached glyph for key");
                    if *cached_glyph != positioned_glyph.key {
                        glyphs.insert((key, glyph));
                    } else {
                        removed_glyphs.insert((key, hash));
                    }
                    let cached_attributes =
                        cache.attributes.get(&key).expect("no cached attributes");
                    // tolerance check each value to decide if should be replaced ond go to cache.changes.updates
                    // also store in cache if changed
                } else {
                    added_keys.insert(key);
                    adds.insert(key, (glyph_section.area, current_attributes));
                    cache.attributes.insert(key, current_attributes);
                    glyphs.insert((key, glyph));
                    cache.glyphs.insert(key, positioned_glyph.key);
                }
            }
            if !glyphs.is_empty() {
                changes.glyphs.insert(entity, glyphs);
            }
            if !removed_glyphs.is_empty() {
                changes.removed_glyphs.insert(entity, removed_glyphs);
            }
            let keys_to_remove = keys
                .keys
                .difference(&retained_keys)
                .copied()
                .collect::<HashSet<Key>>();
            removes.extend(keys_to_remove);
            if !removes.is_empty() {
                changes.removes.insert(entity, removes);
            }
            if !adds.is_empty() {
                changes.adds.insert(entity, adds);
            }

            keys.keys.extend(added_keys);
        }
    }
}
pub(crate) fn visibility(
    text: Query<(Entity, &Text, &Visibility), (Changed<Visibility>)>,
    mut cache: ResMut<Cache>,
    mut changes: ResMut<Changes>,
) {
    for (entity, text, visibility) in text.iter() {
        if visibility.visible {
            cache.visible_entities.insert(entity);
        } else {
            cache.visible_entities.remove(&entity);
        }
        changes.visibility.insert(entity, *visibility);
    }
}
pub(crate) fn text_entity_changes(
    added: Query<(Entity, &Text, &Scale), (Added<Text>)>,
    removed: RemovedComponents<Text>,
    mut changes: ResMut<Changes>,
    font: Res<Font>,
) {
    for (entity, text, scale) in added.iter() {
        changes.added_text_entities.insert(
            entity,
            (
                text.string.len() as u32,
                Alignment::new(font.character_dimensions('a', scale.px())),
            ),
        );
    }
    for entity in removed.iter() {
        changes.removed_text_entities.insert(entity);
    }
}
pub(crate) fn add_remove_rasterizations(
    mut extraction: ResMut<Extraction>,
    mut renderer: ResMut<Renderer>,
    canvas: Res<Canvas>,
) {
    for (entity, (length, alignment)) in extraction.changes.added_text_entities.drain() {
        let rasterization = Rasterization::new(
            &canvas,
            &renderer.rasterization_bind_group_layout,
            alignment,
            length,
        );
        renderer.rasterizations.insert(entity, rasterization);
    }
    for entity in extraction.changes.removed_text_entities.drain() {
        renderer.rasterizations.remove(&entity);
    }
}
pub(crate) fn reset_extraction(mut extraction: ResMut<Extraction>) {
    extraction.changes.reset();
}
pub(crate) fn integrate_extraction(
    mut extraction: ResMut<Extraction>,
    mut renderer: ResMut<Renderer>,
    canvas: Res<Canvas>,
) {
    for (entity, visibility) in extraction.changes.visibility.iter() {
        if visibility.visible {
            renderer.visible_text_entities.insert(*entity);
        } else {
            renderer.visible_text_entities.remove(entity);
        }
    }
    for (entity, section) in extraction.changes.bounds.iter() {
        renderer
            .rasterizations
            .get_mut(entity)
            .expect("no rasterization for entity")
            .bounds
            .replace(*section);
    }
    for entity in extraction.changes.removed_bounds.iter() {
        renderer
            .rasterizations
            .get_mut(entity)
            .expect("no rasterization for entity")
            .bounds = None;
    }
    for (entity, removes) in extraction.changes.removes.iter() {
        let rasterization = renderer
            .rasterizations
            .get_mut(&entity)
            .expect("no rasterization");
        for key in removes.iter() {
            rasterization.instances.remove(*key);
        }
    }
    for (entity, adds) in extraction.changes.adds.iter() {
        let rasterization = renderer
            .rasterizations
            .get_mut(&entity)
            .expect("no rasterization");
        for (key, (area, attributes)) in adds.iter() {
            let tex_coords = rasterization
                .tex_coords
                .get(
                    rasterization
                        .keyed_glyph_hashes
                        .get(key)
                        .expect("no glyph hash"),
                )
                .expect("no tex coords");
            rasterization
                .instances
                .add(*key, Instance::new(*attributes, *area, *tex_coords));
        }
    }
    for (entity, updates) in extraction.changes.updates.iter() {
        let rasterization = renderer
            .rasterizations
            .get_mut(&entity)
            .expect("no rasterization");
        for (key, attributes) in updates.iter() {
            rasterization.instances.update(*key, *attributes);
        }
    }
    for (_entity, mut rasterization) in renderer.rasterizations.iter_mut() {
        rasterization.instances.write(&canvas);
    }
}
pub(crate) fn grow(extraction: Res<Extraction>, renderer: ResMut<Renderer>, canvas: Res<Canvas>) {
    // if projected glyphs larger - grow
    // if projected instances larger - grow
}
impl Attach for Renderer {
    fn attach(engen: &mut Engen) {
        engen
            .compute
            .startup
            .schedule
            .add_system_to_stage(Stage::Before, compute_setup);
        engen
            .compute
            .main
            .schedule
            .add_system_to_stage(Stage::After, text_entity_changes);
        engen
            .compute
            .main
            .schedule
            .add_system_to_stage(Stage::After, visibility);
        engen
            .compute
            .main
            .schedule
            .add_system_to_stage(Stage::Last, emit);
        engen
            .render
            .startup
            .schedule
            .add_system_to_stage(Stage::Before, render_setup);
        engen
            .render
            .main
            .schedule
            .add_system_to_stage(Stage::First, add_remove_rasterizations);
        engen
            .render
            .main
            .schedule
            .add_system_to_stage(Stage::Before, grow);
        engen.render.main.schedule.add_system_to_stage(
            Stage::During,
            rasterize.label("rasterization").before("integration"),
        );
        engen
            .render
            .main
            .schedule
            .add_system_to_stage(Stage::During, integrate_extraction.label("integration"));
        engen
            .render
            .main
            .schedule
            .add_system_to_stage(Stage::Last, reset_extraction);
    }
}
#[derive(Resource)]
pub(crate) struct Extraction {
    pub(crate) changes: Changes,
}
impl Extraction {
    pub(crate) fn new() -> Self {
        Self {
            changes: Changes::new(),
        }
    }
}
impl Render for Renderer {
    fn extract(compute: &mut Task, render: &mut Task)
    where
        Self: Sized,
    {
        let mut changes = compute
            .container
            .get_resource_mut::<Changes>()
            .expect("no text changes attached");
        let mut extraction_changes = &mut render
            .container
            .get_resource_mut::<Extraction>()
            .expect("no extraction attached")
            .changes;
        extraction_changes.adds = changes.adds.drain().collect();
        extraction_changes.updates = changes.updates.drain().collect();
        extraction_changes.removes = changes.removes.drain().collect();
        extraction_changes.glyphs = changes.glyphs.drain().collect();
        extraction_changes.bounds = changes.bounds.drain().collect();
        extraction_changes.added_text_entities = changes.added_text_entities.drain().collect();
        extraction_changes.removed_text_entities = changes.removed_text_entities.drain().collect();
        extraction_changes.removed_glyphs = changes.removed_glyphs.drain().collect();
    }

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
        for (entity, rasterization) in self.rasterizations.iter() {
            if rasterization.instances.count() > 0 && self.visible_text_entities.contains(entity) {
                render_pass_handle
                    .0
                    .set_vertex_buffer(1, rasterization.instances.gpu.slice(..));
                render_pass_handle
                    .0
                    .set_bind_group(2, &rasterization.bind_group, &[]);
                if let Some(bound) = &rasterization.bounds {
                    render_pass_handle.0.set_scissor_rect(
                        bound.position.x as u32,
                        bound.position.y as u32,
                        bound.area.width as u32,
                        bound.area.height as u32,
                    );
                }
                render_pass_handle.0.draw(
                    0..GLYPH_AABB.len() as u32,
                    0..rasterization.instances.count() as u32,
                );
            }
        }
    }
}
