use crate::{
    Area, Color, Coordinate, EnableVisibility, InterfaceContext, Key, Layer, NumericalContext,
    Position, VisibleSection,
};
use bevy_ecs::prelude::{Bundle, Component};
use fontdue::layout::{CoordinateSystem, GlyphPosition, Layout, WrapStyle};
use std::collections::{HashMap, HashSet};
#[derive(Bundle)]
pub struct TextRequest {
    #[bundle]
    pub coord: Coordinate<InterfaceContext>,
    pub text: Text,
    pub scale_alignment: TextScaleAlignment,
    pub color: Color,
    pub wrap_style: TextWrapStyle,
    pub(crate) visibility: EnableVisibility,
    pub(crate) placer: Placer,
    pub(crate) placement: Placement,
    pub(crate) filtered_placement: FilteredPlacement,
    pub(crate) cache: Cache,
    pub(crate) difference: Difference,
}
impl TextRequest {
    pub fn new<Coord: Into<Coordinate<InterfaceContext>>, S: Into<String>, C: Into<Color>>(
        coord: Coord,
        text: S,
        scale_alignment: TextScaleAlignment,
        color: C,
        wrap_style: TextWrapStyle,
    ) -> Self {
        Self {
            coord: coord.into(),
            text: Text(text.into()),
            scale_alignment,
            color: color.into(),
            wrap_style,
            visibility: EnableVisibility::new(),
            placer: Placer(Layout::new(CoordinateSystem::PositiveYDown)),
            placement: Placement(vec![]),
            filtered_placement: FilteredPlacement(vec![]),
            cache: Cache::new(),
            difference: Difference::new(),
        }
    }
}
#[derive(Component)]
pub struct TextWrapStyle(pub WrapStyle);
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
#[derive(Component, Clone)]
pub struct Text(pub String);
#[derive(Component, Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub struct TextScale(pub u32);
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
#[derive(Component, Copy, Clone, Eq, Hash, PartialEq)]
pub enum TextScaleAlignment {
    Small,
    Medium,
    Large,
}
impl TextScaleAlignment {
    pub const TEXT_SCALE_ALIGNMENT_GUIDE: [u32; 3] = [15, 18, 22];
}
#[derive(Component)]
pub struct TextLetterDimensions(pub Area<InterfaceContext>);
#[derive(Hash, Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Debug)]
pub struct TextGridLocation {
    pub x: u32,
    pub y: u32,
}

impl TextGridLocation {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}
#[derive(Component)]
pub struct TextGridPlacement(pub HashMap<TextGridLocation, Key>);
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
    pub(crate) area: Area<InterfaceContext>,
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
            area: Area::default(),
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
