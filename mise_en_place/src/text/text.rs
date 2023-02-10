use std::collections::{HashMap, HashSet};

use bevy_ecs::prelude::{Bundle, Component};

use crate::coord::{Depth, Position, Section};
use crate::text::cache::Cache;
use crate::text::difference::Difference;
use crate::text::glyph::Key;
use crate::text::place::Placer;
use crate::text::scale::TextScaleAlignment;
use crate::visibility::VisibleSection;
use crate::{Color, PositionAdjust};

pub struct TextPartition {
    pub characters: String,
    pub metadata: PartitionMetadata,
}

impl TextPartition {
    pub fn new<S: Into<String>>(characters: S, metadata: PartitionMetadata) -> Self {
        Self {
            characters: characters.into(),
            metadata,
        }
    }
}

#[derive(Component)]
pub struct Text {
    pub partitions: Vec<TextPartition>,
}

impl Text {
    pub fn new(partitions: Vec<TextPartition>) -> Self {
        Self { partitions }
    }
    pub fn length(&self) -> u32 {
        let mut len = 0;
        for part in self.partitions.iter() {
            len += part.characters.len();
        }
        len as u32
    }
}

#[derive(Bundle)]
pub struct TextBundle {
    pub text: Text,
    pub position: Position,
    pub depth: Depth,
    pub scale_alignment: TextScaleAlignment,
    pub(crate) offset: TextOffset,
    pub(crate) placer: Placer,
    pub(crate) cache: Cache,
    pub(crate) difference: Difference,
}

impl TextBundle {
    pub fn new<T: Into<Text>, P: Into<Position>, D: Into<Depth>>(
        text: T,
        position: P,
        depth: D,
        scale_alignment: TextScaleAlignment,
    ) -> Self {
        let position = position.into();
        let depth = depth.into();
        Self {
            text: text.into(),
            position,
            depth,
            scale_alignment,
            offset: TextOffset::default(),
            placer: Placer::new(),
            cache: Cache::new(position, depth, VisibleSection::new(Section::default())),
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

#[derive(Copy, Clone)]
pub struct PartitionMetadata {
    pub color: Color,
    pub mods: u32, //bit_flag for italic/bold/...
}

impl PartitionMetadata {
    pub fn new<C: Into<Color>>(color: C, mods: u32) -> Self {
        Self {
            color: color.into(),
            mods,
        }
    }
}
