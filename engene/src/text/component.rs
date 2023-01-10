use crate::instance::{CachedKeys, EntityKey};
use crate::text::GlyphOffset;
use bevy_ecs::prelude::{Component, Entity, Query};
#[derive(Component)]
pub struct Text {}
#[derive(Component)]
pub struct Placer {}
pub fn emit_requests(
    text: Query<(
        Entity,
        &Text,
        &mut CachedKeys<EntityKey<GlyphOffset>>,
        &mut Placer,
    )>,
) {
    // iterate placer glyphs ond see if cached keys has value for offset,
}
