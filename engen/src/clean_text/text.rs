use bevy_ecs::prelude::{Bundle, Component};
use crate::{Color, Depth, Position, Visibility};
use crate::clean_text::cache::Cache;
use crate::clean_text::extraction::Difference;
use crate::clean_text::place::Placer;
use crate::clean_text::scale::Scale;

#[derive(Component)]
pub struct Text {
    pub string: String,
}
impl Text {
    pub fn new<T: AsRef<&'static str>>(string: T) -> Self {
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
