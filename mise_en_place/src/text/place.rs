use std::collections::HashSet;

use bevy_ecs::prelude::Component;
use fontdue::layout::{CoordinateSystem, GlyphPosition, LayoutSettings, TextStyle, WrapStyle};

use crate::{Color, LetterStyle};
use crate::instance::key::{Key, KeyFactory};
use crate::text::font::MonoSpacedFont;
use crate::text::render_group::TextBound;
use crate::text::scale::TextScale;
use crate::text::text::{LetterMetadata, Text};

#[derive(Component)]
pub(crate) struct Placer {
    layout: fontdue::layout::Layout<LetterMetadata>,
    unfiltered_placement: Vec<(Key, GlyphPosition<LetterMetadata>)>,
    filtered_placement: Vec<(Key, GlyphPosition<LetterMetadata>)>,
}

pub type WrapStyleExpt = WrapStyle;

#[derive(Component)]
pub struct WrapStyleComponent(pub WrapStyle);

impl Placer {
    pub(crate) fn new() -> Self {
        Self {
            layout: fontdue::layout::Layout::new(CoordinateSystem::PositiveYDown),
            unfiltered_placement: vec![],
            filtered_placement: vec![],
        }
    }
    pub(crate) fn place(
        &mut self,
        text: &Text,
        scale: &TextScale,
        font: &MonoSpacedFont,
        wrap_style: &WrapStyleComponent,
        text_bound: Option<&TextBound>,
    ) {
        if let Some(bound) = text_bound {
            self.layout.reset(&LayoutSettings {
                max_width: Option::from(bound.area.width),
                max_height: Option::from(bound.area.height),
                wrap_style: wrap_style.0,
                ..LayoutSettings::default()
            });
        } else {
            self.layout.reset(&LayoutSettings {
                wrap_style: wrap_style.0,
                ..LayoutSettings::default()
            });
        }
        for text_line in text.lines.iter() {
            for letter in text_line.letters.iter() {
                let mut tmp = [0u8; 4];
                self.layout.append(
                    font.font_slice(),
                    &TextStyle::with_user_data(
                        letter.character.encode_utf8(&mut tmp),
                        scale.px(),
                        MonoSpacedFont::index(),
                        letter.metadata,
                    ),
                );
            }
            self.layout.append(
                font.font_slice(),
                &TextStyle::with_user_data(
                    "\n",
                    scale.px(),
                    MonoSpacedFont::index(),
                    LetterMetadata::new(Color::OFF_WHITE, LetterStyle::REGULAR),
                ),
            );
        }
        let mut key_factory = KeyFactory::new();
        self.unfiltered_placement = self
            .layout
            .glyphs()
            .iter()
            .map(|g| (key_factory.generate(), *g))
            .collect::<Vec<(Key, GlyphPosition<LetterMetadata>)>>();
        self.filtered_placement = self.unfiltered_placement.clone();
    }
    pub(crate) fn unfiltered_placement(&self) -> &Vec<(Key, GlyphPosition<LetterMetadata>)> {
        &self.unfiltered_placement
    }
    pub(crate) fn filtered_placement(&self) -> &Vec<(Key, GlyphPosition<LetterMetadata>)> {
        &self.filtered_placement
    }
    pub(crate) fn filter_placement(&mut self, filter_queue: HashSet<Key>) {
        self.filtered_placement
            .retain(|(key, _)| !filter_queue.contains(key));
    }
    pub(crate) fn reset_filtered(&mut self) {
        self.filtered_placement = self.unfiltered_placement.clone();
    }
}
