use bevy_ecs::prelude::{Bundle, Component};
use bitflags::bitflags;
use fontdue::layout::WrapStyle;

use crate::{Color, Location, Visibility, WrapStyleComponent};
use crate::coord::{Section, UIView};
use crate::text::cache::Cache;
use crate::text::difference::Difference;
use crate::text::place::Placer;
use crate::text::scale::TextScaleAlignment;
use crate::visibility::VisibleSection;

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

pub struct TextLine {
    pub letters: Vec<Letter>,
}

impl TextLine {
    pub fn new<L: Into<Letter>>(mut letters: Vec<L>) -> Self {
        let mut letters = letters.drain(..).map(|l| l.into()).collect::<Vec<Letter>>();
        letters.retain(|letter| letter.character != '\n');
        Self { letters }
    }
}

impl<S: Into<String>, C: Into<Color>, M: Into<LetterStyle>> From<(S, C, M)> for TextLine {
    fn from(value: (S, C, M)) -> Self {
        let string = value.0.into();
        let color = value.1.into();
        let style = value.2.into();
        let letters = string
            .chars()
            .map(|c| Letter::new(c, color, style))
            .collect::<Vec<Letter>>();
        TextLine::new(letters)
    }
}

#[derive(Component)]
pub struct Text {
    pub lines: Vec<TextLine>,
}

impl Text {
    pub fn new<TP: Into<TextLine>>(mut lines: Vec<TP>) -> Self {
        Self {
            lines: lines
                .drain(..)
                .map(|tp| tp.into())
                .collect::<Vec<TextLine>>(),
        }
    }
    pub fn length(&self) -> u32 {
        let mut len = 0;
        for part in self.lines.iter() {
            len += part.letters.len();
        }
        len as u32
    }
}

#[derive(Bundle)]
pub struct TextBundle {
    pub text: Text,
    #[bundle]
    pub location: Location<UIView>,
    pub scale_alignment: TextScaleAlignment,
    pub(crate) placer: Placer,
    pub(crate) cache: Cache,
    pub(crate) difference: Difference,
    pub(crate) visibility: Visibility,
    pub(crate) wrap_style: WrapStyleComponent,
}

impl TextBundle {
    pub fn new<T: Into<Text>, L: Into<Location<UIView>>>(
        text: T,
        location: L,
        scale_alignment: TextScaleAlignment,
    ) -> Self {
        let location = location.into();
        Self {
            text: text.into(),
            location,
            scale_alignment,
            placer: Placer::new(),
            cache: Cache::new(
                location.position,
                location.depth,
                VisibleSection::new(Section::default()),
            ),
            difference: Difference::new(),
            visibility: Visibility::new(),
            wrap_style: WrapStyleComponent(WrapStyle::Letter),
        }
    }
}
