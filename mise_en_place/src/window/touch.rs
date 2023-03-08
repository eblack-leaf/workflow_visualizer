use std::collections::HashMap;

use bevy_ecs::prelude::Resource;
use winit::event::{ElementState, MouseButton};

use crate::{DeviceView, Position};

#[derive(Copy, Clone, Default)]
pub struct Click {
    pub origin: Position<DeviceView>,
    pub current: Option<Position<DeviceView>>,
    pub end: Option<Position<DeviceView>>,
}

impl Click {
    pub fn new<PD: Into<Position<DeviceView>>>(origin: PD) -> Self {
        let position = origin.into();
        Self {
            origin: position,
            current: Some(position),
            end: None,
        }
    }
}

pub type Finger = u32;

#[derive(Resource)]
pub struct TouchAdapter {
    pub(crate) primary: Option<Finger>,
    pub(crate) tracked: HashMap<Finger, Click>,
}

impl TouchAdapter {
    pub(crate) fn new() -> Self {
        Self {
            primary: None,
            tracked: HashMap::new(),
        }
    }
    pub fn primary(&self) -> Option<Finger> {
        self.primary.clone()
    }
    pub fn touches(&self) -> &HashMap<Finger, Click> {
        &self.tracked
    }
    pub fn primary_touch(&self) -> Option<Click> {
        match self.primary {
            None => None,
            Some(prime) => Some(*self.tracked.get(&prime).unwrap()),
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum ClickEventType {
    OnPress,
    OnMove,
    OnRelease,
    Cancelled,
}

#[derive(Copy, Clone)]
pub struct ClickEvent {
    pub ty: ClickEventType,
    pub click: Click,
}

impl ClickEvent {
    pub fn new(ty: ClickEventType, click: Click) -> Self {
        Self { ty, click }
    }
}

#[derive(Resource)]
pub struct MouseAdapter {
    pub(crate) location: Option<Position<DeviceView>>,
    pub(crate) button_cache: HashMap<MouseButton, ElementState>,
    pub(crate) clicks: HashMap<MouseButton, Click>,
}

#[allow(unused)]
pub type ElementStateExpt = ElementState;
#[allow(unused)]
pub type MouseButtonExpt = MouseButton;

impl MouseAdapter {
    pub(crate) fn new() -> Self {
        Self {
            location: None,
            button_cache: HashMap::new(),
            clicks: HashMap::new(),
        }
    }
    pub fn location(&self) -> Option<Position<DeviceView>> {
        self.location.clone()
    }
    pub fn clicks(&self) -> &HashMap<MouseButton, Click> {
        &self.clicks
    }
}
