use crate::{Attach, DeviceContext, Engen, Position};
use bevy_ecs::prelude::{Bundle, Commands, Component, EventReader, Query, ResMut, Resource};
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
#[derive(Bundle, Copy, Clone)]
pub struct Touchable {}
#[derive(Copy, Clone)]
pub struct RegisteredTouch {
    pub origin: Touch,
    pub current: Touch,
    pub end: Option<Touch>,
    pub cancelled: bool,
}
impl RegisteredTouch {
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
    mut touch_listeners: Query<()>,
) {
    let new_touch = event_reader.iter().cloned().collect::<Vec<TouchEvent>>();
    let mut cancelled_events = new_touch.clone();
    cancelled_events.retain(|c| c.ty == TouchType::Cancelled);
    let is_cancelled = !cancelled_events.is_empty();
    if !is_cancelled {
        for touch in new_touch {
            match touch.ty {
                TouchType::OnPress => {
                    primary_touch.touch.replace(RegisteredTouch::new(touch.touch));
                }
                TouchType::OnMove => {
                    if let Some(prime) = primary_touch.touch .as_mut(){
                        prime.current = touch.touch;
                    }
                }
                TouchType::OnRelease => {
                    if let Some(prime) = primary_touch.touch.as_mut() {
                        prime.end.replace(touch.touch);
                    }
                }
                _ => {}
            }
        }
    }
}
#[derive(Component, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Interactor(pub u32);
impl Interactor {
    pub fn from_button(button: MouseButton) -> Self {
        Self(match button {
            MouseButton::Left => 0u32,
            _ => { 1u32 }
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
    pub tracked: HashMap<MouseButton, (ElementState, Option<RegisteredTouch>)>,
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
