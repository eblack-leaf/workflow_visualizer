use bevy_ecs::prelude::{Bundle, Component};

use crate::Color;
use crate::coord::{Depth, Position, ScaledSection, Section};
use crate::ecs_text::cache::Cache;
use crate::ecs_text::difference::Difference;
use crate::ecs_text::place::Placer;
use crate::ecs_text::scale::TextScaleAlignment;
use crate::visibility::{Visibility, VisibleSection};

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
