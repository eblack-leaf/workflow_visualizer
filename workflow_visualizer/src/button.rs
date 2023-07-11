use bevy_ecs::prelude::{
    Added, Bundle, Changed, Commands, Component, Entity, IntoSystemConfig, Or, Query, Without,
};
use bevy_ecs::query::With;

use crate::grid::RawMarker;
use crate::grid::ResponsiveGridView;
use crate::{
    Area, Attach, BundlePlacement, Color, CurrentlyPressed, Icon, IconId, IconScale,
    InterfaceContext, Layer, Panel, PanelType, Position, Section, SyncPoint, Text,
    TextScaleAlignment, TextValue, TextWrapStyle, ToggleState, Touchable, Visualizer,
};

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
    touchable: Touchable,
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
            touchable: Touchable::on_press(),
        }
    }
}
#[derive(Component, Copy, Clone)]
pub enum ButtonType {
    Press,
    Toggle,
}
pub(crate) fn spawn(
    mut buttons: Query<
        (
            Entity,
            &Layer,
            &BackgroundColor,
            &Color,
            &IconId,
            &TextValue,
            &mut PanelEntity,
            &mut IconEntity,
            &mut TextEntity,
        ),
        Added<PanelEntity>,
    >,
    mut cmd: Commands,
) {
    for (
        entity,
        layer,
        background_color,
        color,
        icon_id,
        button_text,
        mut panel_entity,
        mut icon_entity,
        mut text_entity,
    ) in buttons.iter_mut()
    {
        let panel = cmd
            .spawn(Panel::new(
                PanelType::BorderedPanel,
                *layer,
                background_color.0,
                *color,
            ))
            .id();
        let icon = cmd
            .spawn(Icon::new(
                icon_id.clone(),
                IconScale::Small,
                *layer - Layer::from(1),
                *color,
            ))
            .id();
        let text = cmd
            .spawn(Text::new(
                *layer - Layer::from(1),
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
pub(crate) fn color_invert(
    buttons: Query<
        (
            &CurrentlyPressed,
            &ToggleState,
            &ButtonType,
            &Color,
            &BackgroundColor,
            &PanelEntity,
            &IconEntity,
            &TextEntity,
        ),
        Or<(Changed<CurrentlyPressed>, Changed<ToggleState>)>,
    >,
    mut color_inverters: Query<(&mut Color), Without<PanelEntity>>,
) {
    for (pressed, toggle, button_type, foreground, background, panel, icon, text) in buttons.iter()
    {
        let mut inverted = false;
        match button_type {
            ButtonType::Press => {
                if pressed.currently_pressed() {
                    inverted = true;
                }
            }
            ButtonType::Toggle => {
                if toggle.toggled() {
                    inverted = true;
                }
            }
        }
        let panel_color_adjust = if inverted { *foreground } else { background.0 };
        let foreground_element_color_adjust = if inverted { background.0 } else { *foreground };
        if let Some(panel_entity) = panel.0 {
            if let Ok(mut panel_color) = color_inverters.get_mut(panel_entity) {
                if *panel_color != panel_color_adjust {
                    *panel_color = panel_color_adjust;
                }
            }
        }
        if let Some(icon_entity) = icon.0 {
            if let Ok(mut icon_color) = color_inverters.get_mut(icon_entity) {
                if *icon_color != foreground_element_color_adjust {
                    *icon_color = foreground_element_color_adjust;
                }
            }
        }
        if let Some(text_entity) = text.0 {
            if let Ok(mut text_color) = color_inverters.get_mut(text_entity) {
                if *text_color != foreground_element_color_adjust {
                    *text_color = foreground_element_color_adjust;
                }
            }
        }
    }
}

pub(crate) fn placement(
    buttons: Query<
        (
            &Position<InterfaceContext>,
            &Area<InterfaceContext>,
            &PanelEntity,
            &IconEntity,
            &TextEntity,
            &TextValue,
        ),
        Or<(
            Changed<Position<InterfaceContext>>,
            Changed<Area<InterfaceContext>>,
        )>,
    >,
    mut listeners: Query<
        (&mut Position<InterfaceContext>, &mut Area<InterfaceContext>),
        Without<PanelEntity>,
    >,
) {
    for (button_pos, button_area, panel_ref, icon_ref, text_ref, button_text) in buttons.iter() {
        if let Some(panel_entity) = panel_ref.0 {
            if let Ok((mut pos, mut area)) = listeners.get_mut(panel_entity) {
                *pos = *button_pos;
                *area = *button_area;
            }
        }
        let section = Section::new(*button_pos, *button_area);
        let center = section.center();
        let (text_placement, icon_placement) = if button_text.0.is_empty() {
            (
                None,
                Position::new(center.x - 13f32 / 2f32, center.y - 13f32 / 2f32),
            )
        } else {
            let len = button_text.0.len() as f32;
            let x = center.x - 8f32 * (len / 2f32).ceil() - 13f32 / 2f32;
            let y = center.y - 18f32 / 2f32;
            let width = 8f32 * len;
            let height = 18f32;
            let text_section = Section::new((x, y), (width, height));
            let icon_x = text_section.right() + RawMarker::PX;
            let icon_y = text_section.top() + RawMarker::PX;
            (Some(text_section), Position::new(icon_x, icon_y))
        };
        if let Some(icon_entity) = icon_ref.0 {
            if let Ok((mut pos, _)) = listeners.get_mut(icon_entity) {
                *pos = icon_placement;
            }
        }
        if let Some(text_entity) = text_ref.0 {
            if let Ok((mut pos, mut area)) = listeners.get_mut(text_entity) {
                if let Some(placement) = text_placement {
                    *pos = placement.position;
                    *area = placement.area;
                }
            }
        }
    }
}

pub(crate) struct ButtonAttachment;

impl Attach for ButtonAttachment {
    fn attach(visualizer: &mut Visualizer) {
        visualizer.job.task(Visualizer::TASK_MAIN).add_systems((
            spawn.in_set(SyncPoint::Spawn),
            placement.in_set(SyncPoint::SecondaryPlacement),
            color_invert.in_set(SyncPoint::Reconfigure),
        ));
    }
}
