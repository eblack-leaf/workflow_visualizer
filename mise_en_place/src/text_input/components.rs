use bevy_ecs::bundle::Bundle;
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;

use crate::clickable::{ClickListener, Clickable};
use crate::focus::Focus;
use crate::text_input::cursor::CursorIcon;
use crate::text_input::Cursor;
use crate::window::VirtualKeyboardType;
use crate::{Color, Location, TextGridGuide, TextScaleAlignment, UIView, Visibility};

#[derive(Component)]
pub struct TextInputText {
    pub entity: Entity,
}

impl TextInputText {
    pub(crate) fn new(entity: Entity) -> Self {
        Self { entity }
    }
}

#[derive(Component)]
pub(crate) struct MaxCharacters(pub(crate) u32);

#[derive(Bundle)]
pub struct TextInput {
    pub(crate) text_input_text: TextInputText,
    pub(crate) cursor_icon: CursorIcon,
    pub(crate) background_icon: TextBackgroundIcon,
    pub(crate) alignment: TextScaleAlignment,
    pub(crate) grid_guide: TextGridGuide,
    #[bundle]
    pub(crate) location: Location<UIView>,
    #[bundle]
    pub(crate) clickable: Clickable,
    pub(crate) max_characters: MaxCharacters,
    pub(crate) focus: Focus,
    pub(crate) keyboard_type: VirtualKeyboardType,
    pub(crate) cursor: Cursor,
    pub(crate) visibility: Visibility,
    pub(crate) text_color: TextColor,
    pub(crate) background_color: TextBackgroundColor,
}

#[derive(Component, Copy, Clone)]
pub struct TextColor(pub Color);

#[derive(Component, Copy, Clone)]
pub struct TextBackgroundColor(pub Color);

impl TextInput {
    pub(crate) fn new<C: Into<Color>>(
        text_input_text: TextInputText,
        cursor_icon: CursorIcon,
        background_icon: TextBackgroundIcon,
        alignment: TextScaleAlignment,
        bound_guide: TextGridGuide,
        location: Location<UIView>,
        text_color: C,
        text_background_color: C,
    ) -> Self {
        Self {
            text_input_text,
            cursor_icon,
            background_icon,
            alignment,
            grid_guide: bound_guide,
            location,
            clickable: Clickable::new(ClickListener::on_press(), false),
            max_characters: MaxCharacters(
                bound_guide.horizontal_character_max * bound_guide.line_max,
            ),
            focus: Focus::new(),
            keyboard_type: VirtualKeyboardType::Keyboard,
            cursor: Cursor::new(),
            visibility: Visibility::new(),
            text_color: TextColor(text_color.into()),
            background_color: TextBackgroundColor(text_background_color.into()),
        }
    }
}

#[derive(Component, Copy, Clone)]
pub struct TextBackgroundIcon(pub Entity);
