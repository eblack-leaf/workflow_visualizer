use bevy_ecs::prelude::{
    Bundle, Component, Entity, EventReader, IntoSystemDescriptor, Query, Res, ResMut, SystemLabel,
    Without,
};

use crate::{
    ClickEvent, ClickEventType, Depth, Position, ScaleFactor, UIView, Visibility, VisibleSection,
};
use crate::engen::{Attach, Engen};
use crate::engen::FrontEndStages;
use crate::focus::FocusedEntity;

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
    pub(crate) currently_pressed: bool,
    pub(crate) toggle: bool,
    pub(crate) click_location: Option<Position<UIView>>,
}

impl ClickState {
    pub fn new(initial_toggle: bool) -> Self {
        Self {
            clicked: false,
            currently_pressed: false,
            toggle: initial_toggle,
            click_location: None,
        }
    }
    pub fn clicked(&self) -> bool {
        self.clicked
    }
    pub fn toggled(&self) -> bool {
        self.toggle
    }
    pub fn currently_pressed(&self) -> bool {
        self.currently_pressed
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
            focused_entity.entity.take();
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

pub struct ClickableAttachment;

#[derive(SystemLabel)]
pub enum ClickSystems {
    RegisterClick,
}

impl Attach for ClickableAttachment {
    fn attach(engen: &mut Engen) {
        engen.frontend.main.add_system_to_stage(
            FrontEndStages::Prepare,
            register_click.label(ClickSystems::RegisterClick),
        );
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::Last, reset_click);
    }
}
