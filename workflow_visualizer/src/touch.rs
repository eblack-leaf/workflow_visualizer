use crate::{Attach, DeviceContext, Engen, Position};
use bevy_ecs::prelude::{Commands, Component, EventReader, ResMut, Resource};
use std::collections::HashMap;
use winit::event::{ElementState, MouseButton};
#[derive(Copy, Clone)]
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
#[derive(Eq, PartialEq, Copy, Clone)]
pub enum TouchType {
    OnPress,
    OnMove,
    OnRelease,
    Cancelled,
}
pub type Touch = Position<DeviceContext>;
#[derive(Copy, Clone)]
pub struct RegisteredTouch {
    pub origin: Touch,
    pub current: Touch,
    pub end: Option<Touch>,
}
impl RegisteredTouch {
    pub(crate) fn new<T: Into<Touch>>(origin: T) -> Self {
        let origin = origin.into();
        Self {
            origin,
            current: origin,
            end: None,
        }
    }
}
#[derive(Resource)]
pub struct PrimaryTouch {
    pub touch: Option<RegisteredTouch>,
}
impl PrimaryTouch {
    pub(crate) fn new() -> Self {
        Self { touch: None }
    }
}
pub(crate) fn read_events(
    mut event_reader: EventReader<TouchEvent>,
    mut primary_touch: ResMut<PrimaryTouch>,
) {
    let new_clicks = event_reader.iter().cloned().collect::<Vec<TouchEvent>>();
}
#[derive(Component, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Interactor(pub u32);
impl Interactor {
    pub fn from_button(button: MouseButton) -> Self {
        Self(match button {
            MouseButton::Left => 0u32,
            MouseButton::Right => 1u32,
            MouseButton::Middle => 2u32,
            MouseButton::Other(code) => code as u32,
        })
    }
}
#[derive(Resource)]
pub struct TouchAdapter {
    pub tracked: HashMap<Interactor, RegisteredTouch>,
    pub primary: Option<Interactor>,
}
impl TouchAdapter {
    pub(crate) fn new() -> Self {
        Self {
            tracked: HashMap::new(),
            primary: None,
        }
    }
}
pub type CursorLocation = Position<DeviceContext>;
#[derive(Resource)]
pub struct MouseAdapter {
    pub location: Option<CursorLocation>,
    pub tracked: HashMap<MouseButton, ElementState>,
}
impl MouseAdapter {
    pub const PRIMARY_INTERACTOR: Interactor = Interactor(0u32);
    pub(crate) fn new() -> Self {
        Self {
            location: None,
            tracked: HashMap::new(),
        }
    }
}
pub struct TouchAttachment;
impl Attach for TouchAttachment {
    fn attach(engen: &mut Engen) {
        engen
            .frontend
            .container
            .insert_resource(PrimaryTouch::new());
        engen
            .frontend
            .container
            .insert_resource(TouchAdapter::new());
        engen
            .frontend
            .container
            .insert_resource(MouseAdapter::new());
    }
}
