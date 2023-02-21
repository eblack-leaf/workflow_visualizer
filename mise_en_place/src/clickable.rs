use bevy_ecs::prelude::{Bundle, Component, Entity, EventReader, Query, Res, Without};

use crate::{
    Area, Attach, ClickEvent, ClickEventType, Engen, FrontEndStages, Position, ScaleFactor, UIView,
    Visibility, VisibleSection,
};

#[derive(Bundle)]
pub struct Clickable {
    pub(crate) click_state: ClickState,
    pub(crate) click_listener: ClickListener,
}

impl Clickable {
    pub fn new(listener: ClickListener) -> Self {
        Self {
            click_state: ClickState::new(),
            click_listener: listener,
        }
    }
}

#[derive(Component)]
pub struct ClickState {
    pub(crate) clicked: bool,
}

impl ClickState {
    pub fn new() -> Self {
        Self { clicked: false }
    }
    pub fn clicked(&self) -> bool {
        self.clicked
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
            Entity,
            &mut ClickState,
            &Position<UIView>,
            &Area<UIView>,
            &ClickListener,
            &Visibility,
            &VisibleSection,
        ),
        Without<DisableClick>,
    >,
    mut clicks: EventReader<ClickEvent>,
    scale_factor: Res<ScaleFactor>,
) {
    for (entity, mut click_state, position, area, listener, visibility, visible_section) in
        clickables.iter_mut()
    {
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
                            }
                        }
                        ClickEventType::Cancelled => {}
                    }
                }
            }
        }
    }
}

pub(crate) fn reset_click(mut clickables: Query<(&mut ClickState)>) {
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
