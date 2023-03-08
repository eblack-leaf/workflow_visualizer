use bevy_ecs::change_detection::ResMut;
use bevy_ecs::entity::Entity;
use bevy_ecs::event::EventReader;
use bevy_ecs::prelude::{Query, Res, Without};

use crate::clickable::{DisableClick, TrackedClick};
use crate::focus::FocusedEntity;
use crate::{
    ClickEvent, ClickEventType, ClickListener, ClickState, Depth, Position, ScaleFactor,
    Visibility, VisibleSection,
};

pub(crate) fn register_click(
    mut clickables: Query<
        (
            Entity,
            &mut ClickState,
            &ClickListener,
            &Visibility,
            &VisibleSection,
            &Depth,
        ),
        Without<DisableClick>,
    >,
    mut clicks: EventReader<ClickEvent>,
    scale_factor: Res<ScaleFactor>,
    mut focused_entity: ResMut<FocusedEntity>,
) {
    let mut new_clicks = clicks
        .iter()
        .map(|ce| TrackedClick::new(*ce))
        .collect::<Vec<TrackedClick>>();
    for (entity, mut click_state, listener, visibility, visible_section, depth) in
        clickables.iter_mut()
    {
        if visibility.visible() {
            for click in new_clicks.iter_mut() {
                let ui_click_origin = click.click_event.click.origin.to_ui(scale_factor.factor);
                match click.click_event.ty {
                    ClickEventType::OnPress => {
                        if visible_section.section.contains(ui_click_origin) {
                            click_state.currently_pressed = true;
                            if listener.ty == click.click_event.ty {
                                set_grabbed(entity, depth, click);
                            }
                        }
                    }
                    ClickEventType::OnMove => {
                        // need to beef up click semantics to handle multiple listeners and detach clicks that
                        // coincidentally start in bounds but only grabbed after moving off previous grabber
                        // need bucket to detach click from to record relative and buffer on move click setting focus entity
                        // let contains_origin = visible_section.section.contains(ui_click_origin);
                        // let contains_current = visible_section.section.contains(click.click_event.click.current.unwrap().to_ui(scale_factor.factor));
                        // if contains_current && contains_origin {
                        //     set_grabbed(entity, depth, click);
                        // }
                    }
                    ClickEventType::OnRelease => {
                        click_state.currently_pressed = false;
                        let end = click
                            .click_event
                            .click
                            .end
                            .unwrap()
                            .to_ui(scale_factor.factor);
                        let contains_origin = visible_section.section.contains(ui_click_origin);
                        let contains_end = visible_section.section.contains(end);
                        if contains_origin && contains_end {
                            if listener.ty == click.click_event.ty {
                                set_grabbed(entity, depth, click);
                            }
                        }
                    }
                    ClickEventType::Cancelled => {}
                }
            }
        }
    }
    for resolved_click in new_clicks {
        if let Some(grab) = resolved_click.grabbed {
            let (_, mut click_state, _, _, _, _) = clickables.get_mut(grab.0).expect("not there");
            let click_location = match resolved_click.click_event.ty {
                ClickEventType::OnPress => resolved_click
                    .click_event
                    .click
                    .origin
                    .to_ui(scale_factor.factor),
                ClickEventType::OnMove => {
                    // currently not getting events as semantics for moving needs buffer + detach
                    // resolved_click
                    //     .click_event
                    //     .click
                    //     .current
                    //     .unwrap()
                    //     .to_ui(scale_factor.factor)
                    Position::default()
                }
                ClickEventType::OnRelease => resolved_click
                    .click_event
                    .click
                    .end
                    .unwrap()
                    .to_ui(scale_factor.factor),
                ClickEventType::Cancelled => Position::default(),
            };
            click_state.click_location.replace(click_location);
            click_state.clicked = true;
            click_state.toggle = !click_state.toggle;
            match resolved_click.click_event.ty {
                ClickEventType::OnPress | ClickEventType::OnRelease => {
                    focused_entity.entity.replace(grab.0);
                }
                ClickEventType::OnMove => {}
                ClickEventType::Cancelled => {}
            }
        } else {
            if resolved_click.click_event.ty == ClickEventType::OnPress {
                focused_entity.entity.take();
            }
        }
    }
}

fn set_grabbed(entity: Entity, depth: &Depth, click: &mut TrackedClick) {
    if let Some((g_ent, g_depth)) = click.grabbed.as_mut() {
        if *depth < *g_depth {
            *g_ent = entity;
            *g_depth = *depth;
        }
    } else {
        click.grabbed.replace((entity, *depth));
    }
}

pub(crate) fn reset_click(mut clickables: Query<&mut ClickState>) {
    for mut click_state in clickables.iter_mut() {
        click_state.clicked = false;
    }
}
