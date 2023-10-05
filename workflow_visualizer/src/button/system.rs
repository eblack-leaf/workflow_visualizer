use bevy_ecs::change_detection::Res;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{Added, Changed, Commands, Or, Query, RemovedComponents, With, Without};

use crate::bundling::{Despawned, Disabled};
use crate::button::{ButtonBorder, ButtonIcon, ButtonText, IconEntity, PanelEntity, TextEntity};
use crate::icon::Icon;
use crate::snap_grid::{FloatPlacementDescriptor, FloatPlacer, FloatRange, FloatView};
use crate::{
    ActiveInteraction, Area, BackgroundColor, BorderColor, ButtonTag, ButtonType, Color, IconScale,
    InterfaceContext, Layer, MonoSpacedFont, Panel, PanelTag, PanelType, Position, Text, TextScale,
    TextSectionDescriptorKnown, TextValue, TextWrapStyle, Toggled,
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
            &ButtonText,
            &ButtonIcon,
            &mut PanelEntity,
            &mut IconEntity,
            &mut TextEntity,
            &ButtonBorder,
        ),
        Added<PanelEntity>,
    >,
    mut cmd: Commands,
) {
    for (
        _entity,
        layer,
        background_color,
        color,
        button_text,
        button_icon,
        mut panel_entity,
        mut icon_entity,
        mut text_entity,
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
        if let Some(icon) = button_icon.desc.as_ref() {
            let entity = cmd
                .spawn(Icon::new(*icon, 0, *layer - Layer::from(1), *color))
                .id();
            icon_entity.0.replace(entity);
        }
        if let Some(text) = button_text.desc.as_ref() {
            text_entity.0.replace(
                cmd.spawn(Text::new(
                    *layer - Layer::from(1),
                    text.0.clone(),
                    0,
                    *color,
                    TextWrapStyle::letter(),
                ))
                .id(),
            );
        }
        panel_entity.0.replace(panel);
    }
}
pub(crate) fn place(
    mut buttons: Query<
        (&PanelEntity, &IconEntity, &TextEntity, &mut FloatPlacer),
        Or<(Changed<IconEntity>, Changed<TextEntity>)>,
    >,
) {
    for (panel_entity, icon_entity, text_entity, mut placer) in buttons.iter_mut() {
        if let Some(entity) = panel_entity.0 {
            placer.add(
                entity,
                FloatPlacementDescriptor::ViewDesc(FloatView::new(
                    FloatRange::new(0.0.into(), 1.0.into()),
                    FloatRange::new(0.0.into(), 1.0.into()),
                )),
            );
        }
        if icon_entity.0.is_some() && text_entity.0.is_some() {
            let icon_placement = FloatPlacementDescriptor::ViewDesc(FloatView::new(
                FloatRange::new(0.05.into(), 0.3.into()),
                FloatRange::new(0.25.into(), 0.8.into()),
            ));
            placer.add(icon_entity.0.unwrap(), icon_placement);
            placer.add(
                text_entity.0.unwrap(),
                FloatPlacementDescriptor::ViewDesc(FloatView::new(
                    FloatRange::new(0.35.into(), 0.95.into()),
                    FloatRange::new(0.15.into(), 0.85.into()),
                )),
            );
        } else if icon_entity.0.is_some() && text_entity.0.is_none() {
            placer.add(
                icon_entity.0.unwrap(),
                FloatPlacementDescriptor::ViewDesc(FloatView::new(
                    FloatRange::new(0.25.into(), 0.75.into()),
                    FloatRange::new(0.15.into(), 0.9.into()),
                )),
            );
        } else if text_entity.0.is_some() && icon_entity.0.is_none() {
            placer.add(
                text_entity.0.unwrap(),
                FloatPlacementDescriptor::ViewDesc(FloatView::new(
                    FloatRange::new(0.05.into(), 0.95.into()),
                    FloatRange::new(0.1.into(), 0.85.into()),
                )),
            );
        }
    }
}
pub(crate) fn scale_change(
    font: Res<MonoSpacedFont>,
    buttons: Query<
        (
            &IconEntity,
            &TextEntity,
            &Position<InterfaceContext>,
            &Area<InterfaceContext>,
        ),
        (
            With<ButtonTag>,
            Or<(Changed<IconEntity>, Changed<TextEntity>)>,
        ),
    >,
    mut listeners: Query<
        (
            &mut Position<InterfaceContext>,
            &Area<InterfaceContext>,
            Option<&mut TextScale>,
            Option<&mut IconScale>,
            Option<&TextValue>,
        ),
        Without<ButtonTag>,
    >,
) {
    for (icon_entity, text_entity, button_pos, button_area) in buttons.iter() {
        if let Some(entity) = icon_entity.0 {
            if let Ok((mut pos, area, _, scale, _)) = listeners.get_mut(entity) {
                let dim = area.width.min(area.height) as u32;
                *scale.unwrap() = IconScale::Symmetrical(dim);
                if text_entity.0.is_none() {
                    let center_x = pos.x + (dim / 2u32) as f32;
                    let button_center_x = button_pos.x + button_area.width / 2f32;
                    let diff = (button_center_x - center_x).abs();
                    if button_center_x > center_x {
                        pos.x += diff;
                    } else {
                        pos.x -= diff;
                    }
                }
                let center_y = pos.y + (dim / 2u32) as f32;
                let button_center_y = button_pos.y + button_area.height / 2f32;
                let diff = (button_center_y - center_y).abs();
                if button_center_y > center_y {
                    pos.y += diff;
                } else if center_y > button_center_y {
                    pos.y -= diff;
                }
            }
        }
        if let Some(entity) = text_entity.0 {
            if let Ok((mut pos, area, scale, _, text_value)) = listeners.get_mut(entity) {
                let new_scale = font
                    .text_section_descriptor(
                        *pos,
                        TextSectionDescriptorKnown::WidthAndHeight(*area),
                        text_value.unwrap().0.len() as u32,
                    )
                    .scale;
                *scale.unwrap() = new_scale;
                // TODO integrate this into text_section_descriptor to get correct text for bounds
                // usage with pos.y = text_section_desc.top(); from font.text_section_descriptor
                let dims = font.character_dimensions(new_scale.px());
                let expected = button_area.height * 0.75;
                let actual = dims.height;
                if actual < expected {
                    let mut diff = expected - actual;
                    diff /= 2f32;
                    pos.y += diff;
                }
            }
        }
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
    mut color_inverters: Query<&mut Color, Without<PanelEntity>>,
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
            if let Ok((mut listened_color, border)) = color_listeners.get_mut(ent) {
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
    for (_entity, panel_entity, text_entity, icon_entity) in despawned_buttons.iter() {
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
