use crate::coord::{Area, Position};
use fontdue::layout::{CoordinateSystem, Layout, LayoutSettings, TextStyle};
use fontdue::{Font, FontSettings};
use std::collections::HashMap;
use std::path::Path;
#[derive(Copy, Clone)]
pub struct MaxCharacters(pub u32);
#[derive(Copy, Clone, PartialEq, Hash)]
pub struct TextScale {
    pub px: f32,
}
impl From<f32> for TextScale {
    fn from(px: f32) -> Self {
        Self { px }
    }
}
pub struct GlyphWidth(pub f32);
pub struct TextFont {
    pub font_storage: [Font; 1],
    pub glyph_widths: HashMap<TextScale, GlyphWidth>,
}
impl TextFont {
    pub fn new<T: AsRef<Path>>(font_path: T, opt_scale: TextScale) -> Self {
        Self {
            font_storage: [Font::from_bytes(
                include_bytes!(font_path),
                FontSettings {
                    scale: opt_scale.px,
                    ..FontSettings::default()
                },
            )
            .expect("invalid font path")],
            glyph_widths: HashMap::new(),
        }
    }
    pub fn font(&self) -> &Font {
        &self.font_storage[0]
    }
    pub fn index() -> usize {
        0
    }
    pub fn font_slice(&self) -> &[Font] {
        self.font_storage.as_slice()
    }
    pub fn line_height(&self, scale: TextScale) -> f32 {
        self.font()
            .horizontal_line_metrics(scale.px)
            .expect("no line metrics")
            .new_line_size
    }
    pub fn line_width(&self, max_characters: MaxCharacters, scale: TextScale) -> f32 {
        (self.glyph_width(scale) * max_characters.0) as f32
    }
    pub fn text_line_metrics(
        &self,
        max_characters: MaxCharacters,
        scale: TextScale,
    ) -> TextLineMetrics {
        let line_height = self.line_height(scale);
        let line_width = self.line_width(max_characters, scale);
        return TextLineMetrics::new(scale, max_characters, (line_width, line_height));
    }
}
pub struct TextLineMetrics {
    pub scale: TextScale,
    pub max_characters: MaxCharacters,
    pub area: Area,
}
impl TextLineMetrics {
    pub fn new<T: Into<Area>>(scale: TextScale, max_characters: MaxCharacters, area: T) -> Self {
        Self {
            scale,
            max_characters,
            area: area.into(),
        }
    }
}
// String that only captures first line of text
pub struct Text {
    string: String,
}
impl Text {
    pub fn new<T: Into<String>>(string: &T) -> Self {
        let string: String = string.into();
        Self {
            string: string
                .lines()
                .next()
                .expect("no lines in text string input")
                .to_string()
                .replace("\n", ""),
        }
    }
    pub fn string(&self) -> &String {
        &self.string
    }
}
pub struct TextLine {
    pub text: Text,
    pub text_line_metrics: TextLineMetrics,
}
impl TextLine {
    pub const ELLIPSIS: &'static str = "...";
    pub fn create_view<'a>(&self) -> TextLineView<'a> {
        let mut ellipsis_text = None;
        let mut viewable_text = Text::new("");
        if self.text.string().len() > self.text_line_metrics.max_characters.0 as usize {
            let (first, second) = self
                .text
                .text()
                .split_at((self.text_line_metrics.max_characters - Self::ELLIPSIS.len()) as usize);
            viewable_text = Text::new(&first.to_string() + Self::ELLIPSIS);
            ellipsis_text = Some(Text::new(&second.to_string()));
        }
        let style = TextStyle::new(
            viewable_text.string().as_str(),
            self.text_line_metrics.scale.px,
            TextFont::index(),
        );
        TextLineView {
            style,
            viewable_text,
            ellipsis_text,
        }
    }
}
pub struct TextLineView<'a> {
    pub style: TextStyle<'a>,
    pub viewable_text: Text,
    pub ellipsis_text: Option<Text>,
}
pub struct TextLineStack<'a> {
    pub position: Position,
    pub layout: Layout,
    pub line_stack: Vec<TextLine>,
    pub line_stack_views: Vec<TextLineView<'a>>,
}
impl TextLineStack {
    pub fn new(font: &TextFont, position: Position, line_stack: Vec<TextLine>) -> Self {
        let mut line_stack_views = Vec::new();
        Self {
            position,
            layout: {
                let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
                layout.reset(&LayoutSettings {
                    x: position.x,
                    y: position.y,
                    ..LayoutSettings::default()
                });
                for line in line_stack {
                    line_stack_views.push(line.create_view());
                    layout.append(font.font_slice(), &line_stack_views.last().unwrap().style);
                }
                layout
            },
            line_stack,
            line_stack_views,
        }
    }
}
