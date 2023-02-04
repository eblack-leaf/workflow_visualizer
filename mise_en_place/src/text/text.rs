use bevy_ecs::prelude::{Bundle, Component};

use crate::Color;
use crate::coord::{Depth, Position, ScaledSection, Section};
use crate::text::cache::Cache;
use crate::text::difference::Difference;
use crate::text::place::Placer;
use crate::text::scale::{TextScale, TextScaleAlignment};
use crate::visibility::{Visibility, VisibleSection};

#[derive(Component)]
pub struct Text {
    string: String,
    dirty: bool,
}

impl Text {
    pub fn new<T: Into<String>>(string: T) -> Self {
        Self {
            string: string.into(),
            dirty: true,
        }
    }
    pub fn len(&self) -> usize {
        self.string.len()
    }
    pub fn string(&self) -> String {
        self.string.clone()
    }
    pub fn clean(&mut self) {
        self.dirty = false;
    }
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }
    pub fn update(&mut self, string: String) {
        self.string = string;
    }
}

impl From<&'static str> for Text {
    fn from(value: &'static str) -> Self {
        Self::new(value.to_string())
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
