use bevy_ecs::prelude::{Bundle, Component, Entity};

pub(crate) use attachment::ButtonAttachment;

use crate::{
    Color, IconHandle, IconScale, Interactable, InterfaceContext, Layer, Section, Tag, TextScale,
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
    icon_id: IconHandle,
    button_text: TextValue,
    section: Section<InterfaceContext>,
    color: Color,
    background_color: BackgroundColor,
    panel_entity: PanelEntity,
    icon_entity: IconEntity,
    text_entity: TextEntity,
    interactable: Interactable,
    scaling: Scaling,
    border: ButtonBorder,
}

#[derive(Component, Copy, Clone)]
pub enum ButtonBorder {
    Border,
    None,
}

#[derive(Component, Copy, Clone)]
pub struct Scaling {
    pub text: TextScale,
    pub icon: IconScale,
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
    pub fn new<
        L: Into<Layer>,
        C: Into<Color>,
        S: Into<String>,
        H: Into<IconHandle>,
        TS: Into<TextScale>,
        IS: Into<IconScale>,
    >(
        button_type: ButtonType,
        layer: L,
        foreground_color: C,
        background_color: C,
        handle: H,
        button_text: S,
        text_scale: TS,
        icon_scale: IS,
        border: ButtonBorder,
    ) -> Self {
        Self {
            tag: ButtonTag::new(),
            layer: layer.into(),
            button_type,
            icon_id: handle.into(),
            button_text: TextValue(button_text.into()),
            section: Section::default(),
            color: foreground_color.into(),
            background_color: BackgroundColor(background_color.into()),
            panel_entity: PanelEntity(None),
            icon_entity: IconEntity(None),
            text_entity: TextEntity(None),
            interactable: Interactable::default(),
            scaling: Scaling {
                text: text_scale.into(),
                icon: icon_scale.into(),
            },
            border,
        }
    }
}

#[derive(Component, Copy, Clone)]
pub enum ButtonType {
    Press,
    Toggle,
}
