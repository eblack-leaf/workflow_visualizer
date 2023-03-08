use bevy_ecs::bundle::Bundle;
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;

use crate::{ClickEvent, ClickEventType, Depth, Position, UIView};

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
pub struct DisableClick {}

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
