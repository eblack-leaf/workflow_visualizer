use std::collections::{HashMap, HashSet};
use std::num::NonZeroU32;

use fontdue::Metrics;

use crate::canvas::Canvas;
use crate::coord::{Area, Position, Section};
use crate::text::coords::Coords;
use crate::text::font::MonoSpacedFont;
use crate::text::glyph::{Glyph, GlyphId, Key};
use crate::CanvasOptions;

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
    pub(crate) texture_width: u32,
    pub(crate) texture_height: u32,
    pub(crate) texture_view: wgpu::TextureView,
    pub(crate) block: Area,
    pub(crate) dimension: u32,
    pub(crate) free: HashSet<Location>,
    pub(crate) glyphs: HashMap<GlyphId, (Coords, Area, Location)>,
    pub(crate) references: HashMap<GlyphId, Reference>,
    pub(crate) write: HashMap<Location, (Coords, Area, Bitmap)>,
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
            texture_width,
            texture_height,
            texture_view,
            block,
            dimension,
            free,
            glyphs: HashMap::new(),
            references: HashMap::new(),
            write: HashMap::new(),
        }
    }
    pub(crate) fn remove_glyph(&mut self, glyph_id: GlyphId) {
        self.references
            .get_mut(&glyph_id)
            .expect("no references for glyph id")
            .decrement();
    }
    pub(crate) fn add_glyph(&mut self, glyph: Glyph, font: &MonoSpacedFont) {
        if self.glyphs.contains_key(&glyph.id) {
            self.increment_reference(glyph.id);
            return;
        }
        self.references.insert(glyph.id, Reference::new());
        self.increment_reference(glyph.id);
        let rasterization = font.font().rasterize(glyph.character, glyph.scale.px());
        let glyph_area = (rasterization.0.width, rasterization.0.height).into();
        let (coords, location) = self.place(rasterization);
        self.glyphs.insert(glyph.id, (coords, glyph_area, location));
    }
    pub(crate) fn write(&mut self, canvas: &Canvas) {
        let mut write = self
            .write
            .drain()
            .collect::<Vec<(Location, (Coords, Area, Bitmap))>>();
        for (location, (coords, glyph_area, bitmap)) in write {
            self.write_texture(canvas, location, coords, glyph_area, bitmap);
        }
    }
    pub(crate) fn read_glyph_info(&self, glyph_id: GlyphId) -> (Coords, Area) {
        let glyph_info = self.glyphs.get(&glyph_id).expect("no glyph for id");
        (glyph_info.0, glyph_info.1)
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
            view_formats: &[wgpu::TextureFormat::R8Unorm],
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
        _coords: Coords,
        glyph_area: Area,
        bitmap: Bitmap,
    ) {
        let position = self.position_from(location);
        let image_copy_texture = wgpu::ImageCopyTexture {
            texture: &self.texture,
            mip_level: 0,
            origin: wgpu::Origin3d {
                x: position.x as u32,
                y: position.y as u32,
                z: 0,
            },
            aspect: wgpu::TextureAspect::All,
        };
        let image_data_layout = wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: NonZeroU32::new(glyph_area.width as u32),
            rows_per_image: NonZeroU32::new(glyph_area.height as u32),
        };
        let extent = wgpu::Extent3d {
            width: glyph_area.width as u32,
            height: glyph_area.height as u32,
            depth_or_array_layers: 1,
        };
        canvas.queue.write_texture(
            image_copy_texture,
            bitmap.as_slice(),
            image_data_layout,
            extent,
        );
    }
    fn queue_write(
        &mut self,
        location: Location,
        coords: Coords,
        glyph_area: Area,
        bitmap: Bitmap,
    ) {
        self.write.insert(location, (coords, glyph_area, bitmap));
    }
    fn place(&mut self, rasterization: (Metrics, Bitmap)) -> (Coords, Location) {
        let location = self.next();
        let position = self.position_from(location);
        let glyph_area: Area = (rasterization.0.width, rasterization.0.height).into();
        let section = Section::new(position, glyph_area);
        let coords = self.coords(section);
        self.queue_write(location, coords, glyph_area, rasterization.1);
        (coords, location)
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
        let (_coords, _area, location) = self
            .glyphs
            .remove(&glyph_id)
            .expect("no glyph for glyph id");
        self.free.insert(location);
        self.references.remove(&glyph_id);
    }
    fn dimension(unique_glyphs: u32) -> u32 {
        let mut logical_dimension = (unique_glyphs as f32).sqrt() as u32;
        while logical_dimension.pow(2) < unique_glyphs {
            logical_dimension += 1;
        }
        logical_dimension
    }
    fn hardware_max_check(texture_width: u32, texture_height: u32) {
        let hardware_max = CanvasOptions::default()
            .web_align()
            .limits
            .max_texture_dimension_2d;
        if texture_width > hardware_max {
            panic!("requested larger than possible texture")
        }
        if texture_height > hardware_max {
            panic!("requested larger than possible texture")
        }
    }
    fn texture_dimensions(block: Area, unique_glyphs: u32) -> (u32, u32, u32) {
        let dimension = Self::dimension(unique_glyphs);
        let texture_width: u32 = dimension * block.width as u32;
        let texture_height: u32 = dimension * block.height as u32;
        Self::hardware_max_check(texture_width, texture_height);
        (dimension, texture_width, texture_height)
    }
    fn coords(&self, glyph_section: Section) -> Coords {
        let normalized_position = Position::new(
            glyph_section.position.x / self.texture_width as f32,
            glyph_section.position.y / self.texture_height as f32,
        );
        let normalized_area = Area::new(
            glyph_section.width() / self.texture_width as f32,
            glyph_section.height() / self.texture_height as f32,
        );
        let normalized_section = Section::new(normalized_position, normalized_area);
        Coords::new(
            normalized_section.left(),
            normalized_section.top(),
            normalized_section.right(),
            normalized_section.bottom(),
        )
    }
}
