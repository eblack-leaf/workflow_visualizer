use bevy_ecs::prelude::{Bundle, Component, Entity, EventReader, Query, Res, ResMut, Without};

use crate::{ClickEvent, ClickEventType, Depth, ScaleFactor, Visibility, VisibleSection};
use crate::engen::{Attach, Engen};
use crate::engen::FrontEndStages;
use crate::focus::FocusedEntity;
use crate::signal::Signal;

#[derive(Bundle)]
pub struct Clickable {
    pub(crate) click_state: ClickState,
    pub(crate) click_listener: ClickListener,
}

impl Clickable {
    pub fn new(listener: ClickListener, initial_toggle: bool) -> Self {
        Self {
            click_state: ClickState::new(initial_toggle),
            click_listener: listener,
        }
    }
}

#[derive(Component)]
pub struct ClickState {
    pub(crate) clicked: bool,
    pub(crate) toggle: bool,
}

impl ClickState {
    pub fn new(initial_toggle: bool) -> Self {
        Self {
            clicked: false,
            toggle: initial_toggle,
        }
    }
    pub fn clicked(&self) -> bool {
        self.clicked
    }
    pub fn toggled(&self) -> bool {
        self.toggle
    }
}

#[derive(Component)]
pub struct ClickListener {
    pub ty: ClickEventType,
}

impl ClickListener {
    pub fn on_press() -> Self {
        Self {
            ty: ClickEventType::OnPress,
        }
    }
    pub fn on_release() -> Self {
        Self {
            ty: ClickEventType::OnRelease,
        }
    }
}

#[derive(Component)]
pub(crate) struct DisableClick {}

pub(crate) struct TrackedClick {
    pub(crate) grabbed: Option<(Entity, Depth)>,
    pub(crate) click_event: ClickEvent,
}

impl TrackedClick {
    pub(crate) fn new(click_event: ClickEvent) -> Self {
        Self {
            grabbed: None,
            click_event,
        }
    }
}

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
    mut focused_entity: ResMut<Signal<FocusedEntity>>,
) {
    let mut new_clicks = clicks.iter().map(|ce| TrackedClick::new(*ce)).collect::<Vec<TrackedClick>>();
    for (entity, _, listener, visibility, visible_section, depth) in clickables.iter() {
        if visibility.visible() {
            for click in new_clicks.iter_mut() {
                if listener.ty == click.click_event.ty {
                    match click.click_event.ty {
                        ClickEventType::OnPress => {
                            if visible_section
                                .section
                                .contains(click.click_event.click.origin.to_ui(scale_factor.factor))
                            {
                                if let Some((g_ent, g_depth)) = click.grabbed.as_mut() {
                                    if *depth < *g_depth {
                                        *g_ent = entity;
                                        *g_depth = *depth;
                                    }
                                } else {
                                    click.grabbed.replace((entity, *depth));
                                }
                            }
                        }
                        ClickEventType::OnMove => {}
                        ClickEventType::OnRelease => {
                            let origin = click.click_event.click.origin.to_ui(scale_factor.factor);
                            let end = click.click_event.click.end.unwrap().to_ui(scale_factor.factor);
                            let contains_origin = visible_section.section.contains(origin);
                            let contains_end = visible_section.section.contains(end);
                            if contains_origin && contains_end {
                                if let Some((g_ent, g_depth)) = click.grabbed.as_mut() {
                                    if depth < g_depth {
                                        *g_ent = entity;
                                        *g_depth = *depth;
                                    }
                                } else {
                                    click.grabbed.replace((entity, *depth));
                                }
                            }
                        }
                        ClickEventType::Cancelled => {}
                    }
                }
            }
        }
    }
    for resolved_click in new_clicks {
        if let Some(grab) = resolved_click.grabbed {
            let (_, mut click_state, _, _, _, _) = clickables.get_mut(grab.0).expect("not there");
            click_state.clicked = true;
            click_state.toggle = !click_state.toggle;
            focused_entity.emit(FocusedEntity::new(Some(grab.0)));
        } else {
            focused_entity.emit(FocusedEntity::new(None));
        }
    }
}

pub(crate) fn reset_click(mut clickables: Query<&mut ClickState>) {
    for mut click_state in clickables.iter_mut() {
        click_state.clicked = false;
    }
}

pub struct ClickablePlugin;

impl Attach for ClickablePlugin {
    fn attach(engen: &mut Engen) {
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::PreProcess, register_click);
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::Last, reset_click);
    }
}
