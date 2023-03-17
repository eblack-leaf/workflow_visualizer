use std::cmp::Ordering;
use std::collections::HashSet;

use bevy_ecs::prelude::Component;
use fontdue::layout::{CoordinateSystem, GlyphPosition, LayoutSettings, TextStyle, WrapStyle};

use crate::instance::key::{Key, KeyFactory};
use crate::text::font::MonoSpacedFont;
use crate::text::render_group::TextBound;
use crate::text::scale::TextScale;
use crate::text::text::LetterMetadata;
use crate::{Letter, TextBuffer, TextGridLocation};

#[derive(Component)]
pub(crate) struct Placer {
    layout: fontdue::layout::Layout<LetterMetadata>,
    unfiltered_placement: Vec<(Key, GlyphPosition<LetterMetadata>)>,
    filtered_placement: Vec<(Key, GlyphPosition<LetterMetadata>)>,
}



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
        text: &TextBuffer,
        scale: &TextScale,
        font: &MonoSpacedFont,
        wrap_style: &WrapStyleComponent,
        text_bound: &TextBound,
    ) {
        self.layout.reset(&LayoutSettings {
            max_width: Option::from(text_bound.area.width),
            max_height: Option::from(text_bound.area.height),
            wrap_style: wrap_style.0,
            ..LayoutSettings::default()
        });
        let mut letters = text.letters.clone();
        let mut letters = letters.drain().collect::<Vec<(TextGridLocation, Letter)>>();
        letters.sort_by(|lhs, rhs| -> Ordering {
            if lhs.0.y > rhs.0.y {
                return Ordering::Greater;
            } else if lhs.0.y < rhs.0.y {
                return Ordering::Less;
            } else if lhs.0.x > rhs.0.x {
                return Ordering::Greater;
            } else if lhs.0.x < rhs.0.x {
                return Ordering::Less;
            }
            Ordering::Equal
        });
        for (_loc, letter) in letters {
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
