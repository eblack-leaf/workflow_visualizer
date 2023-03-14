use std::collections::HashMap;

use bevy_ecs::prelude::{Bundle, Component};
use bitflags::bitflags;
use fontdue::layout::WrapStyle;

use crate::coord::InterfaceContext;
use crate::text::cache::Cache;
use crate::text::difference::Difference;
use crate::text::place::Placer;
use crate::text::scale::TextScaleAlignment;
use crate::text::WrapStyleComponent;
use crate::visibility::{EnableVisibility, VisibleSection};
use crate::{Color, Location, TextGridDescriptor};

bitflags! {
    pub struct LetterStyle: u32 {
        const REGULAR = 0b00000001;
        const BOLD = 0b00000010;
        const ITALIC = 0b00000100;
        const UNDERLINE = 0b00001000;
    }
}
#[derive(Copy, Clone)]
pub struct LetterMetadata {
    pub color: Color,
    pub style: LetterStyle,
}

impl LetterMetadata {
    pub fn new<C: Into<Color>, LS: Into<LetterStyle>>(color: C, style: LS) -> Self {
        Self {
            color: color.into(),
            style: style.into(),
        }
    }
}
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
#[derive(Copy, Clone)]
pub struct Letter {
    pub character: char,
    pub metadata: LetterMetadata,
}

impl Letter {
    pub fn new<Ch: Into<char>, C: Into<Color>, LS: Into<LetterStyle>>(
        character: Ch,
        color: C,
        style: LS,
    ) -> Self {
        Self {
            character: character.into(),
            metadata: LetterMetadata::new(color.into(), style.into()),
        }
    }
}

#[derive(Component)]
pub struct TextContentView {
    pub start: u32,
    pub end: u32,
    pub initial_color: Color,
}

impl TextContentView {
    pub fn new<C: Into<Color>>(start: u32, end: u32, color: C) -> Self {
        Self {
            start,
            end,
            initial_color: color.into(),
        }
    }
}

#[derive(Component)]
pub struct TextContent {
    pub data: String,
}

impl TextContent {
    pub const MAX_CONTENT_LOAD: u32 = 1000;
    pub fn new<C: Into<String>>(data: C) -> Self {
        Self { data: data.into() }
    }
    pub fn view(&self, content_view: &TextContentView) -> TextViewedContent {
        match self.data.get(
            content_view.start as usize
                ..content_view
                    .end
                    .min(content_view.start + Self::MAX_CONTENT_LOAD)
                    .min(self.data.len() as u32) as usize,
        ) {
            None => TextViewedContent("".to_string()),
            Some(dat) => TextViewedContent(dat.to_string()),
        }
    }
}

#[derive(Component, Clone)]
pub struct TextLineStructure {
    pub letter_count: Vec<u32>,
}

impl TextLineStructure {
    pub fn new(letter_count: Vec<u32>) -> Self {
        Self { letter_count }
    }
}

#[derive(Component)]
pub struct TextBuffer {
    pub letters: HashMap<TextGridLocation, Letter>,
}

impl TextBuffer {
    pub fn new<C: Into<Color>>(
        viewed_content: &TextViewedContent,
        color: C,
        grid_guide: &TextGridDescriptor,
    ) -> Self {
        let mut x = 0;
        let mut y = 0;
        let mut letters = HashMap::new();
        let color = color.into();
        for ch in viewed_content.0.chars() {
            if ch == '\n' || x > grid_guide.horizontal_character_max - 1 {
                x = 0;
                y += 1;
            }
            if y > grid_guide.line_max - 1 {
                break;
            }
            letters.insert(
                TextGridLocation::new(x, y),
                Letter::new(ch, color, LetterStyle::REGULAR),
            );
            x += 1;
        }
        Self { letters }
    }
    pub fn num_letters(&self) -> u32 {
        self.letters.len() as u32
    }
}

#[derive(Component, Clone)]
pub struct TextViewedContent(pub String);

#[derive(Bundle)]
pub struct Text {
    pub content: TextContent,
    pub content_view: TextContentView,
    pub viewed_content: TextViewedContent,
    #[bundle]
    pub location: Location<InterfaceContext>,
    pub scale_alignment: TextScaleAlignment,
    pub grid_guide: TextGridDescriptor,
    pub(crate) placer: Placer,
    pub(crate) cache: Cache,
    pub(crate) difference: Difference,
    #[bundle]
    pub(crate) visibility: EnableVisibility,
    pub(crate) wrap_style: WrapStyleComponent,
    pub(crate) text_buffer: TextBuffer,
}

impl Text {
    pub fn new<
        T: Into<TextContent>,
        CV: Into<TextContentView>,
        L: Into<Location<InterfaceContext>>,
    >(
        content: T,
        content_view: CV,
        location: L,
        scale_alignment: TextScaleAlignment,
        grid_guide: TextGridDescriptor,
    ) -> Self {
        let location = location.into();
        let content = content.into();
        let content_view = content_view.into();
        let viewed_content = content.view(&content_view);
        let cached_viewed_content = viewed_content.0.clone();
        let text_buffer = TextBuffer::new(&viewed_content, content_view.initial_color, &grid_guide);
        Self {
            content,
            content_view,
            viewed_content,
            location,
            scale_alignment,
            grid_guide,
            placer: Placer::new(),
            cache: Cache::new(
                location.position,
                location.layer,
                VisibleSection::new(None),
                cached_viewed_content,
            ),
            difference: Difference::new(),
            visibility: EnableVisibility::new(),
            wrap_style: WrapStyleComponent(WrapStyle::Letter),
            text_buffer,
        }
    }
}
