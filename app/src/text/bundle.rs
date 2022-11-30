use crate::coord::Panel;
use crate::text::scale::Scale;
use crate::text::Text;
use crate::{Color, Depth, Position};
use bevy_ecs::prelude::{Bundle, Component};
#[derive(Component)]
pub struct MaxCharacters(pub u32);
#[derive(Bundle)]
pub struct TextBundle {
    pub text: Text,
    pub position: Position,
    pub depth: Depth,
    pub scale: Scale,
    pub max_characters: MaxCharacters,
    pub color: Color,
}
