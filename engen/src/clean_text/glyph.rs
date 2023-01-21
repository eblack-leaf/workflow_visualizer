use crate::clean_text::scale::Scale;
use bevy_ecs::prelude::{Component, Resource};
use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub(crate) struct Key {
    pub(crate) offset: u32,
}
pub(crate) type GlyphId = fontdue::layout::GlyphRasterConfig;
#[derive(Clone)]
pub(crate) struct Glyph {
    pub(crate) character: char,
    pub(crate) scale: Scale,
    pub(crate) id: GlyphId,
}
