use bevy_ecs::prelude::{Bundle, Component};

use crate::coord::{Depth, Position, Section};
use crate::text::cache::Cache;
use crate::text::difference::Difference;
use crate::text::place::Placer;
use crate::text::scale::TextScaleAlignment;
use crate::visibility::VisibleSection;
use crate::Color;

#[derive(Component)]
pub struct Text {
    pub string: String,
}

impl Text {
    pub fn new<T: Into<String>>(string: T) -> Self {
        Self {
            string: string.into(),
        }
    }
}

#[derive(Bundle)]
pub struct TextBundle {
    pub text: Text,
    pub position: Position,
    pub depth: Depth,
    pub color: Color,
    pub scale_alignment: TextScaleAlignment,
    pub offset: TextOffset,
    pub(crate) placer: Placer,
    pub(crate) cache: Cache,
    pub(crate) difference: Difference,
}

impl TextBundle {
    pub fn new<T: Into<Text>, P: Into<Position>, D: Into<Depth>, C: Into<Color>>(
        text: T,
        position: P,
        depth: D,
        color: C,
        scale_alignment: TextScaleAlignment,
        offset: Option<TextOffset>,
    ) -> Self {
        let position = position.into();
        let depth = depth.into();
        let color = color.into();
        Self {
            text: text.into(),
            position,
            depth,
            color,
            scale_alignment,
            offset: offset.unwrap_or_default(),
            placer: Placer::new(),
            cache: Cache::new(
                position,
                depth,
                color,
                VisibleSection::new(Section::default()),
            ),
            difference: Difference::new(),
        }
    }
}

#[derive(Component, PartialEq, Copy, Clone, Default)]
pub struct TextOffset {
    pub x: f32,
    pub y: f32,
}

impl TextOffset {
    pub fn new<P: Into<Position>>(position: P) -> Self {
        let p = position.into();
        Self { x: p.x, y: p.y }
    }
}
