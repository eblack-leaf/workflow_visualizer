use std::collections::HashSet;

use bevy_ecs::prelude::Component;
use fontdue::layout::{CoordinateSystem, GlyphPosition, TextStyle};

use crate::{Scale, Text};
use crate::text::font::MonoSpacedFont;
use crate::text::glyph::Key;

#[derive(Component)]
pub(crate) struct Placer {
    layout: fontdue::layout::Layout,
    unfiltered_placement: Vec<GlyphPosition>,
    filtered_placement: Vec<GlyphPosition>,
}

impl Placer {
    pub(crate) fn new() -> Self {
        Self {
            layout: fontdue::layout::Layout::new(CoordinateSystem::PositiveYDown),
            unfiltered_placement: vec![],
            filtered_placement: vec![],
        }
    }
    pub(crate) fn place(&mut self, text: &Text, scale: &Scale, font: &MonoSpacedFont) {
        self.layout.clear();
        self.layout.append(
            font.font_slice(),
            &TextStyle::new(text.string().as_str(), scale.px(), MonoSpacedFont::index()),
        );
        self.unfiltered_placement = self.layout.glyphs().clone();
        self.filtered_placement = self.unfiltered_placement.clone();
    }
    pub(crate) fn unfiltered_placement(&self) -> &Vec<GlyphPosition> {
        &self.unfiltered_placement
    }
    pub(crate) fn filtered_placement(&self) -> &Vec<GlyphPosition> {
        &self.filtered_placement
    }
    pub(crate) fn filter_placement(&mut self, mut filter_queue: HashSet<Key>) {
        let mut queue = filter_queue.drain().collect::<Vec<Key>>();
        queue.sort();
        queue.reverse();
        for remove in queue.iter() {
            self.filtered_placement.remove(remove.offset as usize);
        }
    }
    pub(crate) fn reset_filtered(&mut self) {
        self.filtered_placement = self.unfiltered_placement.clone();
    }
}