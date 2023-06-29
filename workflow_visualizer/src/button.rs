use crate::{
    Color, Icon, IconId, IconScale, InterfaceContext, Layer, Panel, PanelType,
    ResponsiveGridView, Section, Text, TextScaleAlignment, TextValue, TextWrapStyle,
};
use bevy_ecs::prelude::{Bundle, Commands, Component, Entity, Query};
#[derive(Bundle, Clone)]
pub struct Button {
    layer: Layer,
    button_type: ButtonType,
    icon_id: IconId,
    button_text: TextValue,
    section: Section<InterfaceContext>,
    color: Color,
    background_color: BackgroundColor,
    panel_entity: PanelEntity,
    icon_entity: IconEntity,
    text_entity: TextEntity,
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
    pub fn new<L: Into<Layer>, C: Into<Color>, S: Into<String>, ID: Into<IconId>>(
        button_type: ButtonType,
        layer: L,
        foreground_color: C,
        background_color: C,
        icon_id: ID,
        button_text: S,
    ) -> Self {
        Self {
            layer: layer.into(),
            button_type,
            icon_id: icon_id.into(),
            button_text: TextValue(button_text.into()),
            section: Section::default(),
            color: foreground_color.into(),
            background_color: BackgroundColor(background_color.into()),
            panel_entity: PanelEntity(None),
            icon_entity: IconEntity(None),
            text_entity: TextEntity(None),
        }
    }
}
#[derive(Component, Copy, Clone)]
pub enum ButtonType {
    Press,
    Toggle,
}
pub(crate) fn spawn(mut buttons: Query<(Entity, &Layer, &BackgroundColor, &Color, &IconId, &TextValue, &mut PanelEntity, &mut IconEntity, &mut TextEntity)>, mut cmd: Commands) {
    for (entity, layer, background_color, color, icon_id, button_text, mut panel_entity, mut icon_entity, mut text_entity) in buttons.iter_mut() {
        let panel = cmd
            .spawn(Panel::new(
                PanelType::Panel,
                *layer,
                background_color.0,
                Color::OFF_WHITE.into(),
            ))
            .id();
        let icon = cmd
            .spawn(Icon::new(
                icon_id.clone(),
                IconScale::Small,
                *layer,
                *color,
            ))
            .id();
        let text = cmd
            .spawn(Text::new(
                *layer,
                button_text.0.clone(),
                TextScaleAlignment::Small,
                *color,
                TextWrapStyle::letter(),
            ))
            .id();
        panel_entity.0.replace(panel);
        icon_entity.0.replace(icon);
        text_entity.0.replace(text);
    }
}
