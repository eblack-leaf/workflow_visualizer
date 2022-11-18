use fontdue::layout::{CoordinateSystem, GlyphPosition, Layout, LayoutSettings, TextStyle};

use crate::coord::{Area, Position};
use crate::text_refactor::font::Font;
use crate::text_refactor::glyphs::Glyphs;
use crate::text_refactor::scale::Scale;
use crate::text_refactor::Text;

pub struct Placement {
    pub layout: fontdue::layout::Layout,
}

impl Placement {
    pub fn new() -> Self {
        Self {
            layout: Layout::new(CoordinateSystem::PositiveYUp),
        }
    }
    pub fn place(
        &mut self,
        font: &Font,
        text: Text,
        scale: Scale,
        position: Position,
        area: Option<Area>,
    ) {
        let mut layout_settings = LayoutSettings {
            x: position.x,
            y: position.y,
            ..LayoutSettings::default()
        };
        if let Some(area) = area {
            layout_settings.max_width = Option::from(area.width);
            layout_settings.max_height = Option::from(area.height);
        }
        self.layout.reset(&layout_settings);
        self.layout.append(
            font.font_slice(),
            &TextStyle::new(text.text.as_str(), scale.px(), Font::index()),
        )
    }
    pub fn glyphs(&self) -> Glyphs {
        // could filter the overrun glyphs
        let positioned_glyphs = self.layout.glyphs();
        let glyphs = Glyphs::new();
        return glyphs;
    }
}
