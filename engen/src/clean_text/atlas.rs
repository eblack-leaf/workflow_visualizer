use std::collections::{HashMap, HashSet};
use std::num::NonZeroU32;

use fontdue::Metrics;
use wgpu::Label;

use crate::{Area, Canvas, Position, Section};
use crate::clean_text::coords::Coords;
use crate::clean_text::font::MonoSpacedFont;
use crate::clean_text::glyph::{Glyph, GlyphId, Key};

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub(crate) struct Location {
    pub(crate) x: u32,
    pub(crate) y: u32,
}

impl Location {
    pub(crate) fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

pub(crate) struct Reference {
    pub(crate) count: u32,
}

impl Reference {
    pub(crate) fn new() -> Self {
        Self { count: 0 }
    }
    pub(crate) fn increment(&mut self) {
        self.count += 1;
    }
    pub(crate) fn decrement(&mut self) {
        let sub_value = 1 * (self.count == 0) as u32;
        self.count -= sub_value;
    }
}

pub(crate) type Bitmap = Vec<u8>;

pub(crate) struct Atlas {
    pub(crate) texture: wgpu::Texture,
    pub(crate) texture_view: wgpu::TextureView,
    pub(crate) block: Area,
    pub(crate) dimension: u32,
    pub(crate) free: HashSet<Location>,
    pub(crate) glyphs: HashMap<GlyphId, (Coords, Location)>,
    pub(crate) references: HashMap<GlyphId, Reference>,
    pub(crate) write: HashMap<Location, (Coords, Bitmap)>,
    pub(crate) font: MonoSpacedFont,
}

impl Atlas {
    pub(crate) fn new(canvas: &Canvas, block: Area, unique_glyphs: u32) -> Self {
        let (dimension, texture_width, texture_height) =
            Self::texture_dimensions(block, unique_glyphs);
        let texture_descriptor = Self::texture_descriptor(texture_width, texture_height);
        let texture = canvas.device.create_texture(&texture_descriptor);
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let free = Self::calc_free(dimension);
        Self {
            texture,
            texture_view,
            block,
            dimension,
            free,
            glyphs: HashMap::new(),
            references: HashMap::new(),
            write: HashMap::new(),
            font: MonoSpacedFont::default(),
        }
    }
    pub(crate) fn remove_glyph(&mut self, glyph_id: GlyphId) {
        self.references
            .get_mut(&glyph_id)
            .expect("no references for glyph id")
            .decrement();
    }
    pub(crate) fn add_glyph(&mut self, key: Key, glyph: Glyph) {
        if self.glyphs.contains_key(&glyph.id) {
            self.increment_reference(glyph.id);
            return;
        }
        self.references.insert(glyph.id, Reference::new());
        self.increment_reference(glyph.id);
        let (coords, location) = self.place(glyph.clone());
        self.glyphs.insert(glyph.id, (coords, location));
    }
    pub(crate) fn write(&mut self, canvas: &Canvas) {
        let mut write = self
            .write
            .drain()
            .collect::<Vec<(Location, (Coords, Bitmap))>>();
        for (location, (coords, bitmap)) in write {
            self.write_texture(canvas, location, coords, bitmap);
        }
    }
    pub(crate) fn read_glyph_coords(&self, glyph_id: GlyphId) -> Coords {
        self.glyphs.get(&glyph_id).expect("no glyph for id").0
    }
    fn texture_descriptor(
        texture_width: u32,
        texture_height: u32,
    ) -> wgpu::TextureDescriptor<'static> {
        wgpu::TextureDescriptor {
            label: Some("texture atlas"),
            size: wgpu::Extent3d {
                width: texture_width,
                height: texture_height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        }
    }
    fn calc_free(dimension: u32) -> HashSet<Location> {
        let mut free = HashSet::new();
        for x in 0..dimension {
            for y in 0..dimension {
                let location = Location::new(x, y);
                free.insert(location);
            }
        }
        free
    }
    fn increment_reference(&mut self, glyph_id: GlyphId) {
        self.references.get_mut(&glyph_id).unwrap().increment();
    }
    fn position_from(&self, location: Location) -> Position {
        (
            location.x * self.block.width as u32,
            location.y * self.block.height as u32,
        )
            .into()
    }
    fn write_texture(
        &mut self,
        canvas: &Canvas,
        location: Location,
        coords: Coords,
        bitmap: Bitmap,
    ) {
        let position = self.position_from(location);
        let area = coords.section().area;
        let image_copy_texture = wgpu::ImageCopyTexture {
            texture: &self.texture,
            mip_level: 1,
            origin: wgpu::Origin3d {
                x: position.x as u32,
                y: position.y as u32,
                z: 0,
            },
            aspect: wgpu::TextureAspect::All,
        };
        let glyph_width = area.width as u32;
        let glyph_height = area.height as u32;
        let image_data_layout = wgpu::ImageDataLayout {
            offset: 1,
            bytes_per_row: NonZeroU32::new(glyph_width),
            rows_per_image: NonZeroU32::new(glyph_height),
        };
        let extent = wgpu::Extent3d {
            width: glyph_width,
            height: glyph_height,
            depth_or_array_layers: 0,
        };
        canvas.queue.write_texture(
            image_copy_texture,
            bytemuck::cast_slice(&bitmap),
            image_data_layout,
            extent,
        );
    }
    fn queue_write(&mut self, location: Location, coords: Coords, bitmap: Bitmap) {
        self.write.insert(location, (coords, bitmap));
    }
    fn place(&mut self, glyph: Glyph) -> (Coords, Location) {
        let location = self.next();
        let position = self.position_from(location);
        let (metrics, bitmap) = self.rasterize(glyph);
        let section = Section::new(position, (metrics.width, metrics.height).into());
        let coords = Coords::new(
            section.left(),
            section.top(),
            section.right(),
            section.bottom(),
        );
        self.queue_write(location, coords, bitmap);
        (coords, location)
    }
    fn rasterize(&mut self, glyph: Glyph) -> (Metrics, Vec<u8>) {
        // optimize by caching rasterizations
        self.font
            .font()
            .rasterize(glyph.character, glyph.scale.px())
    }
    fn next(&mut self) -> Location {
        let location = match self.free.is_empty() {
            true => {
                // needs to grow
                panic!("needs to grow texture")
            }
            false => *self.free.iter().next().expect("no free locations"),
        };
        self.free.remove(&location);
        location
    }
    fn free(&mut self) {
        let mut orphaned_glyphs = HashSet::new();
        for (glyph_id, reference) in self.references.iter() {
            if reference.count == 0 {
                orphaned_glyphs.insert(*glyph_id);
            }
        }
        // retain filter
        // ...
        // free
        for glyph_id in orphaned_glyphs {
            self.free_glyph(glyph_id);
        }
    }
    fn free_glyph(&mut self, glyph_id: GlyphId) {
        let (_coords, location) = self
            .glyphs
            .remove(&glyph_id)
            .expect("no glyph for glyph id");
        self.free.insert(location);
        self.references.remove(&glyph_id);
    }
    fn texture_dimensions(block: Area, unique_glyphs: u32) -> (u32, u32, u32) {
        todo!()
    }
}
