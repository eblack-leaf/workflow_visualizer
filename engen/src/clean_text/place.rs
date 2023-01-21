use std::collections::HashSet;

use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{Changed, Component, Or, Query, Res};
use fontdue::layout::{CoordinateSystem, GlyphPosition, TextStyle};

use crate::clean_text::cache::Cache;
use crate::clean_text::difference::Difference;
use crate::clean_text::font::MonoSpacedFont;
use crate::clean_text::glyph::Key;
use crate::clean_text::scale::Scale;
use crate::clean_text::text::Text;
use crate::{Area, Position, Section, Visibility};

#[derive(Component)]
pub(crate) struct Placer {
    pub(crate) layout: fontdue::layout::Layout,
    pub(crate) placement: Vec<GlyphPosition>,
}

impl Placer {
    pub(crate) fn new() -> Self {
        Self {
            layout: fontdue::layout::Layout::new(CoordinateSystem::PositiveYDown),
            placement: vec![],
        }
    }
}

pub(crate) fn place(
    mut dirty_text: Query<
        (&Text, &Scale, &mut Placer),
        Or<(Changed<Text>, Changed<Scale>, Changed<Visibility>)>,
    >,
    font: Res<MonoSpacedFont>,
) {
    for (text, scale, mut placer) in dirty_text.iter_mut() {
        placer.layout.clear();
        placer.layout.append(
            font.font_slice(),
            &TextStyle::new(text.string.as_str(), scale.px(), MonoSpacedFont::index()),
        );
        placer.placement = placer.layout.glyphs().clone();
    }
}

pub(crate) fn discard_out_of_bounds(
    mut text: Query<
        (&mut Placer, &Area, &mut Cache, &mut Difference),
        Or<(Changed<Placer>, Changed<Area>)>,
    >,
) {
    for (mut placer, area, mut cache, mut difference) in text.iter_mut() {
        let text_section = Section::new((0u32, 0u32).into(), *area);
        let mut placement_removals = HashSet::new();
        for glyph in placer.placement.iter() {
            let key = Key::new(glyph.byte_offset as u32);
            let glyph_section =
                Section::new((0u32, 0u32).into(), (glyph.width, glyph.height).into());
            let within_bounds = text_section.left() < glyph_section.right()
                && text_section.right() > glyph_section.left()
                && text_section.top() < glyph_section.bottom()
                && text_section.bottom() > glyph_section.top();
            if !within_bounds {
                if cache.exists(key) {
                    difference.remove.insert(key);
                    placement_removals.insert(key);
                    cache.remove(key);
                }
            }
        }
        for key in placement_removals {
            placer.placement.remove(key.offset as usize);
        }
    }
}
