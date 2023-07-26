use bevy_ecs::bundle::Bundle;
use bevy_ecs::component::Component;

use crate::{DeviceContext, Position};
use crate::touch::adapter::TouchLocation;

/// Registers a Touch has occurred and metadata
#[derive(Copy, Clone, Debug)]
pub struct TouchEvent {
    pub ty: TouchType,
    pub touch: Touch,
}

impl TouchEvent {
    pub(crate) fn new<T: Into<Touch>>(ty: TouchType, touch: T) -> Self {
        Self {
            ty,
            touch: touch.into(),
        }
    }
}

/// Type of TouchEvent received
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum TouchType {
    OnPress,
    OnMove,
    OnRelease,
    Cancelled,
}

/// Wrapper for Position<DeviceContext>
pub type Touch = Position<DeviceContext>;

/// Enables Touch behaviour for an entity
#[derive(Bundle, Clone)]
pub struct Touchable {
    pub(crate) touched: TouchTrigger,
    pub(crate) touched_state: CurrentlyPressed,
    pub(crate) toggle_state: ToggleState,
    pub(crate) listener: TouchListener,
    pub(crate) touch_location: TouchLocation,
}

impl Touchable {
    pub fn new(listener: TouchListener) -> Self {
        Self {
            touched: TouchTrigger::new(),
            touched_state: CurrentlyPressed::new(),
            toggle_state: ToggleState::new(),
            listener,
            touch_location: TouchLocation(None),
        }
    }
    pub fn on_press() -> Self {
        Self::new(TouchListener::on_press())
    }
    pub fn on_release() -> Self {
        Self::new(TouchListener::on_release())
    }
}

/// Listener for receiving touch behaviour
#[derive(Component, Copy, Clone)]
pub struct TouchListener {
    pub listened_type: ListenableTouchType,
    pub(crate) disabled: bool,
}

impl TouchListener {
    pub fn on_press() -> Self {
        Self {
            listened_type: ListenableTouchType::OnPress,
            disabled: false,
        }
    }
    #[allow(unused)]
    pub fn on_release() -> Self {
        Self {
            listened_type: ListenableTouchType::OnRelease,
            disabled: false,
        }
    }
    pub fn disable(&mut self) {
        self.disabled = true;
    }
    pub fn enable(&mut self) {
        self.disabled = false;
    }
}

/// Whether received a touch internally
#[derive(Component, Copy, Clone)]
pub struct TouchTrigger {
    pub(crate) touched: bool,
}

/// Currently touched or not logically
#[derive(Component, Copy, Clone)]
pub struct CurrentlyPressed {
    pub(crate) currently_pressed: bool,
}

impl CurrentlyPressed {
    pub fn new() -> Self {
        Self {
            currently_pressed: false,
        }
    }
    pub fn currently_pressed(&self) -> bool {
        self.currently_pressed
    }
}

/// Tracker for toggle state
#[derive(Component, Copy, Clone)]
pub struct ToggleState {
    pub(crate) toggle: bool,
}

impl ToggleState {
    pub fn new() -> Self {
        Self { toggle: false }
    }
    pub(crate) fn toggled(&self) -> bool {
        self.toggle
    }
}

impl TouchTrigger {
    pub(crate) fn new() -> Self {
        Self { touched: false }
    }
    pub fn triggered(&self) -> bool {
        self.touched
    }
}

/// Types of Touches that can be listened to
#[allow(unused)]
#[derive(Copy, Clone)]
pub enum ListenableTouchType {
    OnPress,
    OnRelease,
}
