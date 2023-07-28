use std::collections::HashMap;

use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::Resource;
use winit::event::{ElementState, MouseButton};

use crate::touch::component::Touch;
use crate::{DeviceContext, Layer, Position};

/// Where a Touch took place
#[derive(Component, Clone)]
pub struct TouchLocation(pub Option<TrackedTouch>);

/// Touch tracked with origin/current/end/cancelled
#[derive(Copy, Clone)]
pub struct TrackedTouch {
    pub origin: Touch,
    pub current: Touch,
    pub end: Option<Touch>,
    pub cancelled: bool,
}

impl TrackedTouch {
    pub(crate) fn new<T: Into<Touch>>(origin: T) -> Self {
        let origin = origin.into();
        Self {
            origin,
            current: origin,
            end: None,
            cancelled: false,
        }
    }
}

/// Touch that started with the PrimaryInteractor
#[derive(Resource)]
pub struct PrimaryTouch {
    pub touch: Option<TrackedTouch>,
}

impl PrimaryTouch {
    pub(crate) fn new() -> Self {
        Self { touch: None }
    }
}

/// Whether the Touch was grabbed or not
#[derive(Resource)]
pub struct TouchGrabState {
    pub(crate) grab_state: Option<(Entity, Layer)>,
}

impl TouchGrabState {
    pub(crate) fn new() -> Self {
        Self { grab_state: None }
    }
}

/// Identifier for an input activator (finger/mouse button)
#[derive(Component, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Interactor(pub u32);

impl Interactor {
    pub fn from_button(button: MouseButton) -> Self {
        Self(match button {
            MouseButton::Left => 0u32,
            _ => 1u32,
        })
    }
}

/// Adapter to manage touch screens
#[derive(Resource)]
pub struct TouchAdapter {
    pub tracked: HashMap<Interactor, TrackedTouch>,
    pub primary: Option<Interactor>,
}

impl TouchAdapter {
    pub(crate) fn new() -> Self {
        Self {
            tracked: HashMap::new(),
            primary: None,
        }
    }
    pub fn current_primary_info(&self) -> Option<TrackedTouch> {
        if let Some(prime) = self.primary.as_ref() {
            return self.tracked.get(prime).copied();
        }
        None
    }
}

/// Where the Cursor is
pub type CursorLocation = Position<DeviceContext>;

/// Adapter for mice
#[derive(Resource)]
pub struct MouseAdapter {
    pub location: Option<CursorLocation>,
    pub tracked: HashMap<MouseButton, (ElementState, Option<TrackedTouch>)>,
}

impl MouseAdapter {
    pub const PRIMARY_INTERACTOR: Interactor = Interactor(0u32);
    pub const PRIMARY_BUTTON: MouseButton = MouseButton::Left;
    pub(crate) fn new() -> Self {
        Self {
            location: None,
            tracked: HashMap::new(),
        }
    }
    pub fn current_primary_info(&self) -> Option<TrackedTouch> {
        if let Some(info) = self.tracked.get(&Self::PRIMARY_BUTTON) {
            return info.1;
        }
        None
    }
}
