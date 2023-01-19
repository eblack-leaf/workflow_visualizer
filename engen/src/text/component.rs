use crate::text::Scale;
use crate::{Color, Depth, Position};
use bevy_ecs::bundle::Bundle;
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use std::collections::HashSet;

#[derive(Bundle)]
pub struct TextBundle {
    pub text: Text,
    pub scale: Scale,
    pub position: Position,
    pub depth: Depth,
    pub color: Color,
    // auto made
    pub(crate) placer: Placer,
    pub(crate) keys: Keys,
}

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub(crate) struct TextOffset(pub usize);

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub(crate) struct Key {
    pub entity: Entity,
    pub offset: TextOffset,
}

impl Key {
    pub(crate) fn new(entity: Entity, offset: TextOffset) -> Self {
        Self { entity, offset }
    }
}

#[derive(Component)]
pub struct Text {
    pub string: String,
}

#[derive(Component)]
pub(crate) struct Placer {
    pub placer: fontdue::layout::Layout,
}

#[derive(Component)]
pub(crate) struct Keys {
    pub keys: HashSet<Key>,
}
