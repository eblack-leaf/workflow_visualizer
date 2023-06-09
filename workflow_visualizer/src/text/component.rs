use std::collections::{HashMap, HashSet};

use bevy_ecs::change_detection::Mut;
use bevy_ecs::prelude::{Bundle, Component};
use fontdue::layout::{CoordinateSystem, GlyphPosition, Layout, WrapStyle};

use crate::{
    Area, Color, Coordinate, DeviceContext, EnableVisibility, InterfaceContext, Key, Layer,
    NumericalContext, Position, ResponsiveGridView, Section, VisibleSection,
};

/// Entry point to spawn a Text element
#[derive(Bundle)]
pub struct Text {
    pub responsive_content_view: ResponsiveGridView,
    pub layer: Layer,
    pub text: TextValue,
    pub scale_alignment: TextScaleAlignment,
    pub color: Color,
    pub wrap_style: TextWrapStyle,
    pub(crate) visibility: EnableVisibility,
    pub(crate) placer: Placer,
    pub(crate) placement: Placement,
    pub(crate) filtered_placement: FilteredPlacement,
    pub(crate) cache: Cache,
    pub(crate) difference: Difference,
    pub(crate) text_letter_dimensions: TextLetterDimensions,
    pub(crate) text_grid_placement: TextGridPlacement,
    pub(crate) text_line_structure: TextLineStructure,
    pub(crate) text_scale: TextScale,
    pub(crate) section: Section<InterfaceContext>,
}

impl Text {
    pub fn new<R: Into<ResponsiveGridView>, S: Into<String>, C: Into<Color>, L: Into<Layer>>(
        responsive_content_view: R,
        layer: L,
        text: S,
        scale_alignment: TextScaleAlignment,
        color: C,
        wrap_style: TextWrapStyle,
    ) -> Self {
        Self {
            responsive_content_view: responsive_content_view.into(),
            layer: layer.into(),
            text: TextValue(text.into()),
            scale_alignment,
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
            text_line_structure: TextLineStructure(vec![], (0, 0)),
            text_scale: TextScale(TextScaleAlignment::TEXT_SCALE_ALIGNMENT_GUIDE[0]),
            section: Section::default(),
        }
    }
}
/// Whether to wrap by Word or Letter
#[derive(Component)]
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
    pub(crate) fn px(&self) -> f32 {
        self.0 as f32
    }
    pub(crate) fn from_alignment(alignment: TextScaleAlignment, scale_factor: f64) -> Self {
        match alignment {
            TextScaleAlignment::Small => Self(
                (TextScaleAlignment::TEXT_SCALE_ALIGNMENT_GUIDE[0] as f64 * scale_factor) as u32,
            ),
            TextScaleAlignment::Medium => Self(
                (TextScaleAlignment::TEXT_SCALE_ALIGNMENT_GUIDE[1] as f64 * scale_factor) as u32,
            ),
            TextScaleAlignment::Large => Self(
                (TextScaleAlignment::TEXT_SCALE_ALIGNMENT_GUIDE[2] as f64 * scale_factor) as u32,
            ),
        }
    }
}
/// What predetermined size category to assign to the Text
#[derive(Component, Copy, Clone, Eq, Hash, PartialEq)]
pub enum TextScaleAlignment {
    Small,
    Medium,
    Large,
}
impl TextScaleAlignment {
    pub const TEXT_SCALE_ALIGNMENT_GUIDE: [u32; 3] = [15, 18, 22];
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
        let x = (position.x / letter_dimensions.0.width).floor() as u32;
        let y = (position.y / letter_dimensions.0.height).floor() as u32;
        Self::new(x, y)
    }
}
/// Mapping of what glyphs keys are placed where in the TextGrid
#[derive(Component, Debug)]
pub struct TextGridPlacement(pub HashMap<TextGridLocation, Key>);
/// Lengths of the lines present in the Text
#[derive(Component, Debug)]
pub struct TextLineStructure(pub Vec<u32>, pub (u32, u32));

impl TextLineStructure {
    pub(crate) fn from_grid_placement(
        grid_placement: &TextGridPlacement,
        area: &Area<InterfaceContext>,
        letter_dimensions: &TextLetterDimensions,
        scale_factor: f64,
    ) -> Self {
        let mut max_y = 0;
        for key in grid_placement.0.keys() {
            if key.y > max_y {
                max_y = key.y;
            }
        }
        let mut line_counts = vec![];
        for _i in 0..max_y + 1 {
            line_counts.push(0);
        }
        for placed in grid_placement.0.keys() {
            *line_counts.get_mut(placed.y as usize).unwrap() += 1;
        }
        let area = area.to_device(scale_factor);
        let max_x = (area.width / letter_dimensions.0.width).floor() as u32;
        let max_y = (area.height / letter_dimensions.0.height).floor() as u32;
        Self(line_counts, (max_x, max_y))
    }
    pub(crate) fn horizontal_character_max(&self) -> u32 {
        self.1 .0 - (!self.1 .0 == 0) as u32
    }
    pub(crate) fn line_max(&self) -> u32 {
        self.1 .1 - (!self.1 .1 == 0) as u32
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
    pub(crate) area: Option<Area<InterfaceContext>>,
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
            area: None,
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
