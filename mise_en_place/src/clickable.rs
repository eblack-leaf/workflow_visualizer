use bevy_ecs::prelude::{Bundle, Component, EventReader, Query, Res, Without};

use crate::{ClickEvent, ClickEventType, ScaleFactor, Visibility, VisibleSection};
use crate::engen::{Attach, Engen};
use crate::engen::FrontEndStages;

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

pub(crate) fn register_click(
    mut clickables: Query<
        (
            &mut ClickState,
            &ClickListener,
            &Visibility,
            &VisibleSection,
        ),
        Without<DisableClick>,
    >,
    mut clicks: EventReader<ClickEvent>,
    scale_factor: Res<ScaleFactor>,
) {
    for (mut click_state, listener, visibility, visible_section) in clickables.iter_mut() {
        if visibility.visible() {
            for click in clicks.iter() {
                if listener.ty == click.ty {
                    match click.ty {
                        ClickEventType::OnPress => {
                            if visible_section
                                .section
                                .contains(click.click.origin.to_ui(scale_factor.factor))
                            {
                                click_state.clicked = true;
                                click_state.toggle = !click_state.toggle;
                            }
                        }
                        ClickEventType::OnMove => {}
                        ClickEventType::OnRelease => {
                            if visible_section
                                .section
                                .contains(click.click.origin.to_ui(scale_factor.factor))
                                && visible_section
                                .section
                                .contains(click.click.end.unwrap().to_ui(scale_factor.factor))
                            {
                                click_state.clicked = true;
                                click_state.toggle = !click_state.toggle;
                            }
                        }
                        ClickEventType::Cancelled => {}
                    }
                }
            }
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
