use bevy_ecs::prelude::{Bundle, Component};

use crate::{Color, Depth, Position};
use crate::text::cache::Cache;
use crate::text::difference::Difference;
use crate::text::place::Placer;
use crate::text::scale::Scale;
use crate::visibility::Visibility;

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

#[derive(Bundle)]
pub struct TextBundle {
    pub text: Text,
    pub position: Position,
    pub depth: Depth,
    pub color: Color,
    pub scale: Scale,
    pub(crate) placer: Placer,
    pub(crate) cache: Cache,
    pub(crate) difference: Difference,
    pub(crate) visibility: Visibility,
}

impl TextBundle {
    pub fn new(text: Text, position: Position, depth: Depth, color: Color, scale: Scale) -> Self {
        Self {
            text,
            position,
            depth,
            color,
            scale,
            placer: Placer::new(),
            cache: Cache::new(position, depth, color),
            difference: Difference::new(),
            visibility: Visibility::new(),
        }
    }
}
