use std::collections::HashSet;

use bevy_ecs::prelude::Component;
use fontdue::layout::{CoordinateSystem, GlyphPosition, LayoutSettings, TextStyle};
use winit::event::VirtualKeyCode::L;

use crate::text::font::MonoSpacedFont;
use crate::text::glyph::Key;
use crate::text::render_group::TextBound;
use crate::text::scale::TextScale;
use crate::text::text::Text;

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
    pub(crate) fn place(
        &mut self,
        text: &Text,
        scale: &TextScale,
        font: &MonoSpacedFont,
        text_bound: Option<&TextBound>,
    ) {
        if let Some(bound) = text_bound {
            self.layout.reset(&LayoutSettings {
                max_width: Option::from(bound.area.width),
                max_height: Option::from(bound.area.height),
                ..LayoutSettings::default()
            });
        } else {
            self.layout.reset(&LayoutSettings::default());
        }
        self.layout.append(
            font.font_slice(),
            &TextStyle::new(text.string.as_str(), scale.px(), MonoSpacedFont::index()),
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
