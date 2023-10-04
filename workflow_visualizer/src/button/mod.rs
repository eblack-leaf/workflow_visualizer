use bevy_ecs::prelude::{Bundle, Component, Entity};

pub(crate) use attachment::ButtonAttachment;

use crate::icon::IconScale;
use crate::{
    Color, Interactable, InterfaceContext, Layer, ResourceHandle, Section, Tag, TextScale,
    TextValue,
};

mod attachment;
mod system;

pub type ButtonTag = Tag<Button>;

#[derive(Bundle, Clone)]
pub struct Button {
    tag: ButtonTag,
    layer: Layer,
    button_type: ButtonType,
    button_icon: Option<ButtonIcon>,
    button_text: Option<ButtonText>,
    section: Section<InterfaceContext>,
    color: Color,
    background_color: BackgroundColor,
    panel_entity: PanelEntity,
    icon_entity: IconEntity,
    text_entity: TextEntity,
    interactable: Interactable,
    border: ButtonBorder,
}
#[derive(Bundle, Clone)]
pub struct ButtonText {
    pub scale: TextScale,
    pub value: TextValue,
}
#[derive(Bundle)]
pub struct ButtonIcon {
    pub scale: IconScale,
    pub icon: ResourceHandle,
}
#[derive(Component, Copy, Clone)]
pub enum ButtonBorder {
    Border,
    None,
}

#[derive(Component, Copy, Clone)]
pub struct BackgroundColor(pub Color);

#[derive(Component, Copy, Clone)]
pub(crate) struct PanelEntity(pub(crate) Option<Entity>);

#[derive(Component, Copy, Clone)]
pub(crate) struct IconEntity(pub(crate) Option<Entity>);

#[derive(Component, Copy, Clone)]
pub(crate) struct TextEntity(pub(crate) Option<Entity>);

impl Button {
    pub fn new<L: Into<Layer>, C: Into<Color>>(
        button_type: ButtonType,
        layer: L,
        foreground_color: C,
        background_color: C,
        button_text: Option<ButtonText>,
        button_icon: Option<ButtonIcon>,
        border: ButtonBorder,
    ) -> Self {
        Self {
            tag: ButtonTag::new(),
            layer: layer.into(),
            button_type,
            button_icon,
            button_text,
            section: Section::default(),
            color: foreground_color.into(),
            background_color: BackgroundColor(background_color.into()),
            panel_entity: PanelEntity(None),
            icon_entity: IconEntity(None),
            text_entity: TextEntity(None),
            interactable: Interactable::default(),
            border,
        }
    }
}

#[derive(Component, Copy, Clone)]
pub enum ButtonType {
    Press,
    Toggle,
}
