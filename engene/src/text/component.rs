use crate::color::Color;
use crate::coord::{Depth, Position};
use crate::instance::{CachedKeys, EntityKey, Request};
use crate::text::extractor::Extractor;
use crate::text::font::Font;
use crate::text::scale::Scale;
use crate::text::{GlyphOffset, RequestData};
use bevy_ecs::prelude::{Bundle, Commands, Component, Entity, Query, Res, ResMut};
use fontdue::layout::{CoordinateSystem, TextStyle};
#[derive(Bundle)]
pub struct TextBundle {
    pub text: Text,
    pub scale: Scale,
    pub position: Position,
    pub depth: Depth,
    pub color: Color,
    pub placer: Placer,
    pub cached_keys: CachedKeys<EntityKey<GlyphOffset>>,
}
impl TextBundle {
    pub fn new(text: Text, scale: Scale, position: Position, depth: Depth, color: Color) -> Self {
        Self {
            text,
            scale,
            position,
            depth,
            color,
            placer: Placer::new(),
            cached_keys: CachedKeys::new(),
        }
    }
}
#[derive(Component)]
pub struct Text {
    pub string: String,
    pub(crate) updated: bool,
}
impl Text {
    pub fn new(string: String) -> Self {
        Self {
            string,
            updated: true,
        }
    }
}
#[derive(Component)]
pub struct Delete {}
pub fn delete(
    text: Query<(Entity, &Text, &Delete, &CachedKeys<EntityKey<GlyphOffset>>)>,
    mut cmd: Commands,
    mut extractor: ResMut<Extractor>,
) {
    for (entity, text, _delete, cached_keys) in text.iter() {
        // send key removals to text_renderer?
        for key in cached_keys.used_keys.iter() {
            extractor.remove_handler.removes.insert(*key);
        }
        cmd.entity(entity).despawn();
    }
}
#[derive(Component)]
pub struct Placer {
    pub layout: fontdue::layout::Layout,
}
impl Placer {
    pub fn new() -> Self {
        Self {
            layout: fontdue::layout::Layout::new(CoordinateSystem::PositiveYDown),
        }
    }
}
pub fn emit_requests(
    mut text: Query<(
        Entity,
        &mut Text,
        &Position,
        &Depth,
        &Scale,
        &Color,
        &mut CachedKeys<EntityKey<GlyphOffset>>,
        &mut Placer,
    )>,
    font: Res<Font>,
    mut extractor: ResMut<Extractor>,
) {
    // iterate placer glyphs ond see if cached keys has value for offset,
    for (entity, mut text, position, depth, scale, color, mut cached_keys, mut placer) in
        text.iter_mut()
    {
        if text.updated {
            text.updated = false;
            placer.layout.clear();
            placer.layout.append(
                font.font_slice(),
                &TextStyle::new(text.string.as_str(), scale.px(), Font::index()),
            );
            for glyph in placer.layout.glyphs() {
                let offset = GlyphOffset(glyph.byte_offset);
                let key = EntityKey::new(entity, offset);
                let request = Request::new(RequestData::new(
                    glyph.parent,
                    *scale,
                    glyph.key,
                    *position,
                    (glyph.width, glyph.height).into(),
                    *depth,
                    *color,
                ));
                // future opt - use cache here to save extraction and processing time before internal
                // cache check
                extractor.request_handler.requests.insert(key, request);
                cached_keys.used_keys.insert(key);
            }
        }
    }
}
