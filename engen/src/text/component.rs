use crate::text::Scale;
use crate::{Color, Depth, Position};
use bevy_ecs::bundle::Bundle;
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use std::collections::HashSet;
use fontdue::layout::CoordinateSystem;
use crate::canvas::Visibility;

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
    pub(crate) visibility: Visibility,
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
            keys: Keys::new(),
            visibility: Visibility::new(),
        }
    }
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
impl Text {
    pub fn new(string: String) -> Self {
        Self {
            string
        }
    }
}
#[derive(Component)]
pub(crate) struct Placer {
    pub placer: fontdue::layout::Layout,
}
impl Placer {
    pub(crate) fn new() -> Self {
        Self {
            placer: fontdue::layout::Layout::new(CoordinateSystem::PositiveYDown),
        }
    }
}
#[derive(Component)]
pub(crate) struct Keys {
    pub keys: HashSet<Key>,
}
impl Keys {
    pub(crate) fn new() -> Self {
        Self {
            keys: HashSet::new(),
        }
    }
}
