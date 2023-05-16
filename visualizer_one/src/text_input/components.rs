use bevy_ecs::bundle::Bundle;
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;

use crate::focus::Focus;
use crate::text_input::cursor::CursorIcon;
use crate::text_input::Cursor;
use crate::touch::{TouchListener, Touchable};
use crate::visibility::EnableVisibility;
use crate::{
    Area, Color, InterfaceContext, Layer, Location, Section, TextLetterDimensions,
    TextLineStructure, TextScale, TextScaleAlignment, ViewArea, ViewPosition, VirtualKeyboardType,
};

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
pub struct MaxCharacters(pub u32);

#[derive(Bundle)]
pub struct TextInput {
    pub(crate) text_input_text: TextInputText,
    pub(crate) cursor_icon: CursorIcon,
    pub(crate) content_panel: TextContentPanel,
    pub(crate) alignment: TextScaleAlignment,
    pub(crate) view_position: ViewPosition,
    pub(crate) view_area: ViewArea,
    pub(crate) layer: Layer,
    #[bundle]
    pub(crate) section: Section<InterfaceContext>,
    #[bundle]
    pub(crate) touchable: Touchable,
    pub(crate) max_characters: MaxCharacters,
    pub(crate) focus: Focus,
    pub(crate) keyboard_type: VirtualKeyboardType,
    pub(crate) cursor: Cursor,
    #[bundle]
    pub(crate) visibility: EnableVisibility,
    pub(crate) text_color: TextColor,
    pub(crate) background_color: TextBackgroundColor,
    pub(crate) letter_dimensions: TextLetterDimensions,
    pub(crate) text_scale: TextScale,
}

#[derive(Component, Copy, Clone)]
pub struct TextColor(pub Color);

#[derive(Component, Copy, Clone)]
pub struct TextBackgroundColor(pub Color);

impl TextInput {
    pub(crate) fn new<C: Into<Color>>(
        view_position: ViewPosition,
        view_area: ViewArea,
        layer: Layer,
        text_input_text: TextInputText,
        cursor_icon: CursorIcon,
        content_panel: TextContentPanel,
        alignment: TextScaleAlignment,
        text_color: C,
        text_background_color: C,
        max_characters: MaxCharacters,
    ) -> Self {
        Self {
            text_input_text,
            cursor_icon,
            content_panel,
            alignment,
            view_position,
            view_area,
            layer,
            section: Section::default(),
            touchable: Touchable::new(TouchListener::on_press()),
            max_characters,
            focus: Focus::new(),
            keyboard_type: VirtualKeyboardType::Keyboard,
            cursor: Cursor::new(),
            visibility: EnableVisibility::new(),
            text_color: TextColor(text_color.into()),
            background_color: TextBackgroundColor(text_background_color.into()),
            letter_dimensions: TextLetterDimensions(Area::default()),
            text_scale: TextScale(12u32),
        }
    }
}

#[derive(Component, Copy, Clone)]
pub struct TextContentPanel(pub Entity);
