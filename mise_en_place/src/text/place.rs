use std::collections::HashSet;

use bevy_ecs::prelude::Component;
use fontdue::layout::{CoordinateSystem, GlyphPosition, LayoutSettings, TextStyle, WrapStyle};

use crate::instance::key::{Key, KeyFactory};
use crate::text::font::MonoSpacedFont;
use crate::text::render_group::TextBound;
use crate::text::scale::TextScale;
use crate::text::text::{PartitionMetadata, Text};

#[derive(Component)]
pub(crate) struct Placer {
    layout: fontdue::layout::Layout<PartitionMetadata>,
    unfiltered_placement: Vec<(Key, GlyphPosition<PartitionMetadata>)>,
    filtered_placement: Vec<(Key, GlyphPosition<PartitionMetadata>)>,
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
        for part in text.partitions.iter() {
            self.layout.append(
                font.font_slice(),
                &TextStyle::with_user_data(
                    part.characters.as_str(),
                    scale.px(),
                    MonoSpacedFont::index(),
                    part.metadata,
                ),
            );
        }
        let mut key_factory = KeyFactory::new();
        self.unfiltered_placement = self
            .layout
            .glyphs()
            .iter()
            .map(|g| (key_factory.generate(), *g))
            .collect::<Vec<(Key, GlyphPosition<PartitionMetadata>)>>();
        self.filtered_placement = self.unfiltered_placement.clone();
    }
    pub(crate) fn unfiltered_placement(&self) -> &Vec<(Key, GlyphPosition<PartitionMetadata>)> {
        &self.unfiltered_placement
    }
    pub(crate) fn filtered_placement(&self) -> &Vec<(Key, GlyphPosition<PartitionMetadata>)> {
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
