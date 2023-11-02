use std::collections::{HashMap, HashSet};

use bevy_ecs::prelude::{Bundle, Component};
use fontdue::layout::{CoordinateSystem, GlyphPosition, Layout, WrapStyle};

use crate::{
    Area, Color, DeviceContext, EnableVisibility, InterfaceContext, Key, Layer, MonoSpacedFont,
    NumericalContext, Position, Section, Tag, VisibleSection,
};
pub type TextTag = Tag<Text>;
/// Entry point to spawn a Text element
#[derive(Bundle)]
pub struct Text {
    tag: TextTag,
    pub layer: Layer,
    pub text: TextValue,
    pub color: Color,
    pub wrap_style: TextWrapStyle,
    pub scale: TextScale,
    pub(crate) visibility: EnableVisibility,
    pub(crate) placer: Placer,
    pub(crate) placement: Placement,
    pub(crate) filtered_placement: FilteredPlacement,
    pub(crate) cache: Cache,
    pub(crate) difference: Difference,
    pub(crate) text_letter_dimensions: TextLetterDimensions,
    pub(crate) text_grid_placement: TextGridPlacement,
    pub(crate) text_line_structure: TextLineStructure,
    pub(crate) section: Section<InterfaceContext>,
}

impl Text {
    pub fn new<S: Into<String>, C: Into<Color>, L: Into<Layer>, TS: Into<TextScale>>(
        layer: L,
        text: S,
        scale: TS,
        color: C,
        wrap_style: TextWrapStyle,
    ) -> Self {
        Self {
            tag: TextTag::new(),
            layer: layer.into(),
            text: TextValue(text.into()),
            color: color.into(),
            wrap_style,
            visibility: EnableVisibility::new(),
            placer: Placer(Layout::new(CoordinateSystem::PositiveYDown)),
            placement: Placement(vec![]),
            filtered_placement: FilteredPlacement(vec![]),
            cache: Cache::new(),
            difference: Difference::new(),
            text_letter_dimensions: TextLetterDimensions(Area::default()),
            text_grid_placement: TextGridPlacement(HashMap::new()),
            text_line_structure: TextLineStructure::new(),
            scale: scale.into(),
            section: Section::default(),
        }
    }
}
/// Whether to wrap by Word or Letter
#[derive(Component, Copy, Clone)]
pub struct TextWrapStyle(pub WrapStyle);
impl TextWrapStyle {
    pub fn letter() -> Self {
        TextWrapStyle(WrapStyle::Letter)
    }
    pub fn word() -> Self {
        TextWrapStyle(WrapStyle::Word)
    }
}
pub(crate) type GlyphId = fontdue::layout::GlyphRasterConfig;
#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub(crate) struct Glyph {
    pub(crate) character: char,
    pub(crate) scale: TextScale,
    pub(crate) id: GlyphId,
}

impl Glyph {
    pub(crate) fn new(character: char, scale: TextScale, id: GlyphId) -> Self {
        Self {
            character,
            scale,
            id,
        }
    }
}

/// The text to render
#[derive(Component, Clone)]
pub struct TextValue(pub String);
/// The scale of the Text
#[derive(Component, Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub struct TextScale(pub u32);
impl From<u32> for TextScale {
    fn from(value: u32) -> Self {
        TextScale(value)
    }
}
impl TextScale {
    pub fn px(&self) -> f32 {
        self.0 as f32
    }
}
/// The size of a letter at an alignment
/// This holds `Area<DeviceContext>` as character dimensions are
/// scaled before rasterization to prevent visual artifacts of stretching
#[derive(Component, Copy, Clone)]
pub struct TextLetterDimensions(pub Area<DeviceContext>);
/// where the letter is in the Text Grid
#[derive(Hash, Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Debug)]
pub struct TextGridLocation {
    pub x: u32,
    pub y: u32,
}

impl TextGridLocation {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
    pub fn from_position(
        position: Position<DeviceContext>,
        letter_dimensions: TextLetterDimensions,
    ) -> Self {
        let x = (position.x / letter_dimensions.0.width * MonoSpacedFont::TEXT_HEIGHT_CORRECTION)
            .floor() as u32;
        let y = (position.y / letter_dimensions.0.height * MonoSpacedFont::TEXT_HEIGHT_CORRECTION)
            .floor() as u32;
        Self::new(x, y)
    }
}
/// Mapping of what glyphs keys are placed where in the TextGrid
#[derive(Component, Debug)]
pub struct TextGridPlacement(pub HashMap<TextGridLocation, Key>);
/// Lengths of the lines present in the Text
#[derive(Component, Debug)]
pub struct TextLineStructure(pub Vec<(u32, u32)>);

impl TextLineStructure {
    pub(crate) fn new() -> Self {
        Self(vec![])
    }
}

#[derive(Component)]
pub(crate) struct Placer(pub(crate) Layout);
#[derive(Component)]
pub(crate) struct Placement(pub(crate) Vec<(Key, GlyphPosition<()>)>);
#[derive(Component)]
pub(crate) struct FilteredPlacement(pub(crate) Vec<(Key, GlyphPosition<()>)>);
#[derive(Component)]
pub(crate) struct Cache {
    pub(crate) keys: HashSet<Key>,
    pub(crate) glyphs: HashMap<Key, GlyphId>,
    pub(crate) glyph_position: HashMap<Key, Position<NumericalContext>>,
    pub(crate) glyph_color: HashMap<Key, Color>,
    pub(crate) position: Position<InterfaceContext>,
    pub(crate) layer: Layer,
    pub(crate) visible_section: VisibleSection,
}
impl Cache {
    pub(crate) fn new() -> Self {
        Self {
            keys: HashSet::new(),
            glyphs: HashMap::new(),
            glyph_position: HashMap::new(),
            glyph_color: HashMap::new(),
            position: Position::default(),
            layer: Layer::default(),
            visible_section: VisibleSection::default(),
        }
    }
    pub(crate) fn exists(&self, key: Key) -> bool {
        self.keys.contains(&key)
    }
    pub(crate) fn get_glyph_id(&self, key: Key) -> GlyphId {
        *self.glyphs.get(&key).expect("no glyph id")
    }
    pub(crate) fn remove(&mut self, key: Key) {
        self.keys.remove(&key);
        self.glyphs.remove(&key);
        self.glyph_position.remove(&key);
    }
    pub(crate) fn add(
        &mut self,
        key: Key,
        glyph_id: GlyphId,
        glyph_position: Position<NumericalContext>,
    ) {
        self.keys.insert(key);
        self.glyphs.insert(key, glyph_id);
        self.glyph_position.insert(key, glyph_position);
    }
    pub(crate) fn glyph_position_different(
        &self,
        key: Key,
        glyph_position: Position<NumericalContext>,
    ) -> bool {
        *self
            .glyph_position
            .get(&key)
            .expect("no glyph position for key")
            != glyph_position
    }
    pub(crate) fn glyph_id_different(&self, key: Key, glyph_id: GlyphId) -> bool {
        *self.glyphs.get(&key).expect("no glyph id for key") != glyph_id
    }
}
#[derive(Component, Clone)]
pub(crate) struct Difference {
    pub(crate) position: Option<Position<InterfaceContext>>,
    pub(crate) visible_section: Option<VisibleSection>,
    pub(crate) layer: Option<Layer>,
    pub(crate) glyph_add: HashMap<Key, Glyph>,
    pub(crate) glyph_remove: HashSet<GlyphId>,
    pub(crate) glyph_color_update: HashMap<Key, Color>,
    pub(crate) added: HashMap<Key, Position<NumericalContext>>,
    pub(crate) updated: HashMap<Key, Position<NumericalContext>>,
    pub(crate) remove: HashSet<Key>,
}
impl Difference {
    pub(crate) fn new() -> Self {
        Self {
            position: None,
            visible_section: None,
            layer: None,
            glyph_add: HashMap::new(),
            glyph_remove: HashSet::new(),
            glyph_color_update: HashMap::new(),
            added: HashMap::new(),
            updated: HashMap::new(),
            remove: HashSet::new(),
        }
    }
}
