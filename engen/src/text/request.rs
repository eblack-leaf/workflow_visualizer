use crate::color::Color;
use crate::coord::{Area, Depth, Position};
use crate::instance::EntityKey;
use crate::text::rasterization::GlyphHash;
use crate::text::{rasterization, Font, Scale};
use bevy_ecs::prelude::{Component, Entity, Query, Res, ResMut, Resource};
use fontdue::layout::{CoordinateSystem, LayoutSettings, TextStyle};
use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub struct GlyphOffset(pub(crate) usize);
#[derive(Clone)]
pub struct InstanceRequest {
    pub character: char,
    pub scale: Scale,
    pub hash: GlyphHash,
    pub position: Position,
    pub area: Area,
    pub depth: Depth,
    pub color: Color,
    pub descriptor: Option<rasterization::Descriptor>,
}
impl InstanceRequest {
    pub fn new(
        character: char,
        scale: Scale,
        hash: GlyphHash,
        position: Position,
        area: Area,
        depth: Depth,
        color: Color,
    ) -> Self {
        Self {
            character,
            scale,
            hash,
            position,
            area,
            depth,
            color,
            descriptor: None,
        }
    }
}
#[derive(Component)]
pub struct Placer {
    pub placer: fontdue::layout::Layout,
}
impl Placer {
    pub fn new() -> Self {
        Self {
            placer: fontdue::layout::Layout::new(CoordinateSystem::PositiveYDown),
        }
    }
}
#[derive(Component)]
pub struct Text {
    pub string: String,
    pub scale: Scale,
    pub dirty: bool,
}
impl Text {
    pub fn new(string: String, scale: Scale) -> Self {
        Self {
            string,
            scale,
            dirty: false,
        }
    }
}
#[derive(Resource)]
pub struct InstanceRequests {
    pub requests: HashMap<EntityKey<GlyphOffset>, InstanceRequest>,
}
impl InstanceRequests {
    pub fn new() -> Self {
        Self {
            requests: HashMap::new(),
        }
    }
}
pub fn emit_requests(
    mut text_entities: Query<(Entity, &mut Text, &mut Placer, &Position, &Depth, &Color)>,
    mut requests: ResMut<InstanceRequests>,
    font: Res<Font>,
) {
    for (entity, mut text, mut placer, position, depth, color) in text_entities.iter_mut() {
        if text.dirty {
            placer.placer.clear();
            placer.placer.append(
                font.font_slice(),
                &TextStyle::new(text.string.as_str(), text.scale.px(), Font::index()),
            );
            for glyph in placer.placer.glyphs() {
                requests.requests.insert(
                    EntityKey::new(entity, GlyphOffset(glyph.byte_offset)),
                    InstanceRequest::new(
                        glyph.parent,
                        text.scale,
                        glyph.key,
                        *position + (glyph.x, glyph.y).into(),
                        (glyph.width, glyph.height).into(),
                        *depth,
                        *color,
                    ),
                );
            }
            text.dirty = false;
        }
    }
}
