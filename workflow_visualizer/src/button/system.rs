use bevy_ecs::change_detection::Res;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{Added, Changed, Commands, Or, Query, RemovedComponents, With, Without};

use crate::bundling::{Despawned, Disabled};
use crate::button::{ButtonBorder, IconEntity, PanelEntity, Scaling, TextEntity};
use crate::{
    ActiveInteraction, Area, BackgroundColor, BorderColor, ButtonTag, ButtonType, Color,
    DeviceContext, Icon, IconHandle, InterfaceContext, Layer, MonoSpacedFont, Panel, PanelTag,
    PanelType, Position, RawMarker, ScaleFactor, Section, Text, TextValue, TextWrapStyle, Toggled,
};

pub(crate) fn border_change(
    buttons: Query<(&PanelEntity, &ButtonBorder), Changed<ButtonBorder>>,
    mut panels: Query<&mut PanelType, With<PanelTag>>,
) {
    for (panel_ref, border) in buttons.iter() {
        if let Some(panel) = panel_ref.0 {
            if let Ok(mut panel_type) = panels.get_mut(panel) {
                match border {
                    ButtonBorder::Border => *panel_type = PanelType::BorderedFlat,
                    ButtonBorder::None => *panel_type = PanelType::Flat,
                }
            }
        }
    }
}
pub(crate) fn spawn(
    mut buttons: Query<
        (
            Entity,
            &Layer,
            &BackgroundColor,
            &Color,
            &IconHandle,
            &TextValue,
            &mut PanelEntity,
            &mut IconEntity,
            &mut TextEntity,
            &Scaling,
            &ButtonBorder,
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
        scaling,
        border,
    ) in buttons.iter_mut()
    {
        let panel = cmd
            .spawn(Panel::new(
                match border {
                    ButtonBorder::Border => PanelType::BorderedFlat,
                    ButtonBorder::None => PanelType::Flat,
                },
                *layer,
                background_color.0,
                *color,
            ))
            .id();
        let icon = cmd
            .spawn(Icon::new(
                icon_id.clone(),
                scaling.icon,
                *layer - Layer::from(1),
                *color,
            ))
            .id();
        let text = cmd
            .spawn(Text::new(
                *layer - Layer::from(1),
                button_text.0.clone(),
                scaling.text.0,
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
            &ActiveInteraction,
            &Toggled,
            &ButtonType,
            &Color,
            &BackgroundColor,
            &PanelEntity,
            &IconEntity,
            &TextEntity,
        ),
        Or<(Changed<ActiveInteraction>, Changed<Toggled>)>,
    >,
    mut color_inverters: Query<(&mut Color), Without<PanelEntity>>,
) {
    for (active_interaction, toggle, button_type, foreground, background, panel, icon, text) in
        buttons.iter()
    {
        let mut inverted = false;
        match button_type {
            ButtonType::Press => {
                if active_interaction.active() {
                    inverted = true;
                }
            }
            ButtonType::Toggle => {
                if toggle.active() || active_interaction.active() {
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
            &Scaling,
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
    aligned_fonts: Res<MonoSpacedFont>,
    scale_factor: Res<ScaleFactor>,
) {
    for (button_pos, button_area, panel_ref, icon_ref, text_ref, button_text, scaling) in
        buttons.iter()
    {
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
                Position::new(
                    center.x - scaling.icon.width_px() / 2f32,
                    center.y - scaling.icon.height_px() / 2f32,
                ),
            )
        } else {
            let dimensions = aligned_fonts.character_dimensions(scaling.text.px());
            let logical_dimensions =
                Area::<DeviceContext>::new(dimensions.width, dimensions.height)
                    .to_ui(scale_factor.factor());
            let len = button_text.0.len() as f32;
            let x = center.x - logical_dimensions.width * (len / 2f32).ceil()
                + scaling.icon.width_px() / 2f32
                + RawMarker(2).to_pixel();
            let y = center.y - logical_dimensions.height / 2f32;
            let width = logical_dimensions.width * len;
            let height = logical_dimensions.height;
            let text_section = Section::new((x, y), (width, height));
            let icon_x = text_section.left() - RawMarker(2).to_pixel() - scaling.icon.width_px();
            let icon_y = text_section.top() + RawMarker(1).to_pixel();
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

pub(crate) fn color_forward(
    mut color_listeners: Query<(&mut Color, Option<&mut BorderColor>), Without<ButtonTag>>,
    mut color_deciders: Query<
        (
            &Color,
            &mut BackgroundColor,
            &PanelEntity,
            &IconEntity,
            &TextEntity,
        ),
        (
            With<ButtonTag>,
            Or<(Changed<Color>, Changed<BackgroundColor>)>,
        ),
    >,
) {
    for (color, mut back_color, panel_ent, icon_ent, text_ent) in color_deciders.iter_mut() {
        back_color.0.alpha = color.alpha;
        if let Some(ent) = panel_ent.0 {
            if let Ok((mut listened_color, mut border)) = color_listeners.get_mut(ent) {
                *listened_color = back_color.0;
                border.unwrap().0 = *color;
            }
        }
        if let Some(ent) = icon_ent.0 {
            if let Ok((mut listened_color, _)) = color_listeners.get_mut(ent) {
                *listened_color = *color;
            }
        }
        if let Some(ent) = text_ent.0 {
            if let Ok((mut listened_color, _)) = color_listeners.get_mut(ent) {
                *listened_color = *color;
            }
        }
    }
}

pub(crate) fn secondary_despawn(
    despawned_buttons: Query<(Entity, &PanelEntity, &TextEntity, &IconEntity), With<Despawned>>,
    mut cmd: Commands,
) {
    for (entity, panel_entity, text_entity, icon_entity) in despawned_buttons.iter() {
        if let Some(ent) = panel_entity.0 {
            cmd.entity(ent).despawn();
        }
        if let Some(ent) = text_entity.0 {
            cmd.entity(ent).despawn();
        }
        if let Some(ent) = icon_entity.0 {
            cmd.entity(ent).despawn();
        }
    }
}

pub(crate) fn forward_disable(
    disabled: Query<(&PanelEntity, &TextEntity, &IconEntity), Added<Disabled>>,
    mut cmd: Commands,
) {
    for (panel, text, icon) in disabled.iter() {
        if let Some(ent) = panel.0 {
            cmd.entity(ent).insert(Disabled::default());
        }
        if let Some(ent) = text.0 {
            cmd.entity(ent).insert(Disabled::default());
        }
        if let Some(ent) = icon.0 {
            cmd.entity(ent).insert(Disabled::default());
        }
    }
}

pub(crate) fn remove_disabled(
    enabled: Query<(&PanelEntity, &TextEntity, &IconEntity)>,
    mut removed: RemovedComponents<Disabled>,
    mut cmd: Commands,
) {
    for remove in removed.iter() {
        if let Ok((panel, text, icon)) = enabled.get(remove) {
            if let Some(ent) = panel.0 {
                cmd.entity(ent).remove::<Disabled>();
            }
            if let Some(ent) = text.0 {
                cmd.entity(ent).remove::<Disabled>();
            }
            if let Some(ent) = icon.0 {
                cmd.entity(ent).remove::<Disabled>();
            }
        }
    }
}
