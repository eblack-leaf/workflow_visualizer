use crate::text::extract::Extraction;
use crate::text::Renderer;
use crate::Canvas;
use bevy_ecs::change_detection::ResMut;
use bevy_ecs::prelude::Res;

pub(crate) fn grow(extraction: Res<Extraction>, renderer: ResMut<Renderer>, canvas: Res<Canvas>) {
    // if projected glyphs larger - grow
    // if projected instances larger - grow
}
