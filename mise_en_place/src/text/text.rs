use std::collections::{HashMap, HashSet};

use bevy_ecs::prelude::{Bundle, Component};

use crate::coord::{Depth, Position, Section, UIView};
use crate::key::Key;
use crate::text::cache::Cache;
use crate::text::difference::Difference;
use crate::text::place::Placer;
use crate::text::scale::TextScaleAlignment;
use crate::visibility::VisibleSection;
use crate::{Color, Location, PositionAdjust};

pub struct TextPartition {
    pub characters: String,
    pub metadata: PartitionMetadata,
}

impl TextPartition {
    pub fn new<S: Into<String>, PM: Into<PartitionMetadata>>(characters: S, metadata: PM) -> Self {
        Self {
            characters: characters.into(),
            metadata: metadata.into(),
        }
    }
}

impl<S: Into<String>, PM: Into<PartitionMetadata>> From<(S, PM)> for TextPartition {
    fn from(value: (S, PM)) -> Self {
        Self::new(value.0, value.1)
    }
}

#[derive(Component)]
pub struct Text {
    pub partitions: Vec<TextPartition>,
}

impl Text {
    pub fn new<TP: Into<TextPartition>>(mut partitions: Vec<TP>) -> Self {
        Self {
            partitions: partitions
                .drain(..)
                .map(|tp| tp.into())
                .collect::<Vec<TextPartition>>(),
        }
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
    #[bundle]
    pub location: Location<UIView>,
    pub scale_alignment: TextScaleAlignment,
    pub(crate) placer: Placer,
    pub(crate) cache: Cache,
    pub(crate) difference: Difference,
}

impl TextBundle {
    pub fn new<T: Into<Text>, L: Into<Location<UIView>>>(
        text: T,
        location: L,
        scale_alignment: TextScaleAlignment,
    ) -> Self {
        let location = location.into();
        Self {
            text: text.into(),
            location,
            scale_alignment,
            placer: Placer::new(),
            cache: Cache::new(
                location.position,
                location.depth,
                VisibleSection::new(Section::default()),
            ),
            difference: Difference::new(),
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

impl<C: Into<Color>> From<(C, u32)> for PartitionMetadata {
    fn from(value: (C, u32)) -> Self {
        Self::new(value.0, value.1)
    }
}
