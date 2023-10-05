use bevy_ecs::prelude::{Bundle, Component, Entity};

pub(crate) use attachment::ButtonAttachment;

use crate::snap_grid::FloatPlacer;
use crate::{
    Color, Interactable, InterfaceContext, Layer, ResourceHandle, Section, Tag, TextValue,
};

mod attachment;
mod system;

pub type ButtonTag = Tag<Button>;

#[derive(Bundle)]
pub struct Button {
    tag: ButtonTag,
    layer: Layer,
    button_type: ButtonType,
    button_icon: ButtonIcon,
    button_text: ButtonText,
    section: Section<InterfaceContext>,
    color: Color,
    background_color: BackgroundColor,
    panel_entity: PanelEntity,
    icon_entity: IconEntity,
    text_entity: TextEntity,
    interactable: Interactable,
    border: ButtonBorder,
    float_placer: FloatPlacer,
}

#[derive(Component, Clone)]
pub struct ButtonText {
    pub desc: Option<TextValue>,
}
impl ButtonText {
    pub fn none() -> Self {
        Self { desc: None }
    }
    pub fn some(desc: TextValue) -> Self {
        Self { desc: Some(desc) }
    }
}
#[derive(Component)]
pub struct ButtonIcon {
    pub desc: Option<ResourceHandle>,
}
impl ButtonIcon {
    pub fn none() -> Self {
        Self { desc: None }
    }
    pub fn some(desc: ResourceHandle) -> Self {
        Self { desc: Some(desc) }
    }
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
    pub const TEXT_HEIGHT_FACTOR: f32 = 0.95;
    pub fn new<L: Into<Layer>, C: Into<Color>>(
        button_type: ButtonType,
        layer: L,
        foreground_color: C,
        background_color: C,
        button_text: ButtonText,
        button_icon: ButtonIcon,
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
            float_placer: FloatPlacer::new(),
        }
    }
}

#[derive(Component, Copy, Clone)]
pub enum ButtonType {
    Press,
    Toggle,
}
