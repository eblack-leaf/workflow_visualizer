use crate::text::component::Key;
use crate::text::extract::Extraction;
use crate::text::font::Font;
use crate::text::instance::{InstanceBuffer, TexCoords};
use crate::text::{Renderer, Scale};
use crate::{Area, Canvas, Position, Section};
use bevy_ecs::change_detection::ResMut;
use bevy_ecs::prelude::Res;
use std::collections::{HashMap, HashSet};
use std::num::NonZeroU32;
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupLayout, Extent3d, ImageCopyTexture, ImageDataLayout,
    Origin3d, Texture, TextureAspect, TextureDimension, TextureFormat, TextureUsages, TextureView,
    TextureViewDescriptor,
};

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
        let normalized_position = self.normalize_pos(position);
        let normalized_area = self.normalize_area(&grid_entry);
        self.insert_at(placement_location, &grid_entry);
        self.write_to_texture(canvas, &grid_entry, position);
        self.generate_tex_coords(glyph_hash, normalized_position, normalized_area);
        self.attempt_to_set_next_location(placement_location);
    }
    fn insert_at(&mut self, placement_location: GridLocation, grid_entry: &GridEntry) {
        self.grid.insert(placement_location, grid_entry.clone());
    }
    fn normalize_area(&mut self, grid_entry: &GridEntry) -> Area {
        (
            grid_entry.area.width / self.total_x(),
            grid_entry.area.height / self.total_y(),
        )
            .into()
    }
    fn normalize_pos(&mut self, position: Position) -> Position {
        (position.x / self.total_x(), position.y / self.total_y()).into()
    }
    fn generate_tex_coords(
        &mut self,
        glyph_hash: GlyphHash,
        normalized_position: Position,
        normalized_area: Area,
    ) {
        let tex_coords = TexCoords::new([
            normalized_position.x,
            normalized_position.y,
            normalized_position.x + normalized_area.width,
            normalized_position.y + normalized_area.height,
        ]);
        self.tex_coords.insert(glyph_hash, tex_coords);
    }
    fn attempt_to_set_next_location(&mut self, placement_location: GridLocation) {
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
    fn write_to_texture(&mut self, canvas: &Canvas, grid_entry: &GridEntry, position: Position) {
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
        let block_width = grid_entry.area.width as u32;
        let block_height = grid_entry.area.height as u32;
        let image_data_layout = ImageDataLayout {
            offset: 1,
            bytes_per_row: NonZeroU32::new(block_width),
            rows_per_image: NonZeroU32::new(block_height),
        };
        let extent = Extent3d {
            width: block_width,
            height: block_height,
            depth_or_array_layers: 1,
        };
        canvas.queue.write_texture(
            image_copy_texture,
            bytemuck::cast_slice(&grid_entry.bitmap),
            image_data_layout,
            extent,
        );
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
    fn logical_dimension(initial_supported_instances: u32) -> u32 {
        let mut logical_dimension = (initial_supported_instances as f32).sqrt() as u32;
        while logical_dimension.pow(2) < initial_supported_instances {
            logical_dimension += 1;
        }
        logical_dimension
    }
    pub(crate) fn new(
        canvas: &Canvas,
        bind_group_layout: &wgpu::BindGroupLayout,
        alignment: Alignment,
        initial_supported_instances: u32,
    ) -> Self {
        let instances = InstanceBuffer::new(canvas, initial_supported_instances);
        let logical_dimension = Self::logical_dimension(initial_supported_instances);
        let (texture_width, texture_height) =
            Self::texture_dimensions(&alignment, logical_dimension);
        Self::hardware_max_check(canvas, texture_width, texture_height);
        let (texture, view, bind_group) = Self::create_texture_resources(
            canvas,
            bind_group_layout,
            texture_width,
            texture_height,
        );
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

    fn create_texture_resources(
        canvas: &Canvas,
        bind_group_layout: &BindGroupLayout,
        texture_width: u32,
        texture_height: u32,
    ) -> (Texture, TextureView, BindGroup) {
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
        (texture, view, bind_group)
    }

    fn hardware_max_check(canvas: &Canvas, texture_width: u32, texture_height: u32) {
        let hardware_max = canvas.options.limits.max_texture_dimension_2d;
        if texture_width > hardware_max {
            panic!("requested larger than possible texture")
        }
        if texture_height > hardware_max {
            panic!("requested larger than possible texture")
        }
    }

    fn texture_dimensions(alignment: &Alignment, logical_dimension: u32) -> (u32, u32) {
        let texture_width: u32 = logical_dimension * alignment.dimensions[0] as u32;
        let texture_height: u32 = logical_dimension * alignment.dimensions[1] as u32;
        (texture_width, texture_height)
    }
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
            let rasterization = renderer
                .rasterizations
                .get_mut(&entity)
                .expect("no rasterization for entity");
            rasterization.keyed_glyph_hashes.insert(*key, glyph.hash);
            if rasterization.glyph_locations.contains_key(&glyph.hash) {
                rasterization
                    .references
                    .get_mut(&glyph.hash)
                    .expect("no references")
                    .increment();
            } else {
                let (metrics, bitmap) = font.font().rasterize(glyph.character, glyph.scale.px());
                rasterization.add(
                    &canvas,
                    GridEntry::new(bitmap, (metrics.width, metrics.height).into()),
                    glyph.hash,
                );
                rasterization
                    .references
                    .insert(glyph.hash, GlyphReference::new());
            }
        }
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

pub(crate) type GlyphHash = fontdue::layout::GlyphRasterConfig;

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
