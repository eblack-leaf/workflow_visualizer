use std::collections::HashSet;

use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{Changed, Component, Or, Query, Res};
use fontdue::layout::{GlyphPosition, TextStyle};

use crate::clean_text::font::MonoSpacedFont;
use crate::clean_text::glyph::Key;
use crate::clean_text::scale::Scale;
use crate::clean_text::text::Text;
use crate::{Area, Position, Section, Visibility};

#[derive(Component)]
pub(crate) struct Placer {
    pub(crate) layout: fontdue::layout::Layout,
    pub(crate) placement: Vec<GlyphPosition>,
    pub(crate) out_of_bounds_glyphs: HashSet<Key>,
}

pub(crate) fn place(
    mut dirty_text: Query<
        (Entity, &Text, &Scale, &mut Placer),
        (Or<(Changed<Text>, Changed<Scale>, Changed<Visibility>)>),
    >,
    font: Res<MonoSpacedFont>,
) {
    for (entity, text, scale, mut placer) in dirty_text.iter_mut() {
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
        (Entity, &mut Placer, &Position, &Area),
        (Or<(Changed<Placer>, Changed<Area>)>),
    >,
) {
    // discard glyphs from placer.placement if not in entity section bounds
    for (entity, mut placer, position, area) in text.iter_mut() {
        let section = Section::new(*position, *area);
        placer.out_of_bounds_glyphs.clear();
        for glyph in placer.placement.iter() {
            // check section bounds with glyph bounds - insert into out of bounds if not covered
        }
    }
}
