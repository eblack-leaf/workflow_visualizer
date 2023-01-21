use bevy_ecs::prelude::{Bundle, Component};

use crate::clean_text::cache::Cache;
use crate::clean_text::difference::Difference;
use crate::clean_text::place::Placer;
use crate::clean_text::scale::Scale;
use crate::{Color, Depth, Position, Visibility};

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
