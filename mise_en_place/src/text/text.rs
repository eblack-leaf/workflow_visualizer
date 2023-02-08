use bevy_ecs::prelude::{Bundle, Component};

use crate::coord::{Depth, Position, Section};
use crate::text::cache::Cache;
use crate::text::difference::Difference;
use crate::text::place::Placer;
use crate::text::scale::TextScaleAlignment;
use crate::visibility::VisibleSection;
use crate::{Color, PositionAdjust};

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
    pub(crate) offset: TextOffset,
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
            offset: TextOffset::default(),
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
pub struct TextOffsetAdjustGuide {
    pub characters_to_offset_x: i32,
    pub lines_to_offset_y: i32,
}
impl TextOffsetAdjustGuide {
    pub fn new(characters_to_offset_x: i32, lines_to_offset_y: i32) -> Self {
        Self {
            characters_to_offset_x,
            lines_to_offset_y,
        }
    }
}
#[derive(Component, PartialEq, Copy, Clone, Default)]
pub(crate) struct TextOffsetAdjust {
    pub position_adjust: PositionAdjust,
}
impl TextOffsetAdjust {
    pub(crate) fn new<PA: Into<PositionAdjust>>(adjust: PA) -> Self {
        Self {
            position_adjust: adjust.into(),
        }
    }
}
#[derive(Component, PartialEq, Copy, Clone, Default)]
pub(crate) struct TextOffset {
    pub(crate) position: Position,
}

impl TextOffset {
    pub(crate) fn new<P: Into<Position>>(position: P) -> Self {
        Self {
            position: position.into(),
        }
    }
}
