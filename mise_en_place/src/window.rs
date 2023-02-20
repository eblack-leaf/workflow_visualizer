use std::collections::{HashMap, HashSet};

use bevy_ecs::prelude::{Events, ResMut, Resource};
use winit::event::{AxisId, ElementState, MouseButton};

use crate::coord::DeviceView;
use crate::window::Orientation::{Landscape, Portrait};
use crate::{Area, Attach, BackendStages, Engen, FrontEndStages, Position};

#[derive(Resource)]
pub struct VirtualKeyboardAdapter {
    pub(crate) open: bool,
}

impl VirtualKeyboardAdapter {
    pub(crate) fn new() -> Self {
        Self { open: false }
    }
    pub fn is_open(&self) -> bool {
        self.open
    }
    pub fn open(&mut self) {
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::{prelude::*, JsCast};
            let document = web_sys::window().unwrap().document().unwrap();
            document
                .get_element_by_id("keyboard_trigger")
                .unwrap()
                .dyn_into::<web_sys::HtmlElement>()
                .unwrap()
                .blur()
                .unwrap();
            document
                .get_element_by_id("keyboard_trigger")
                .unwrap()
                .dyn_into::<web_sys::HtmlElement>()
                .unwrap()
                .focus()
                .unwrap();
            self.open = true;
        }
    }
    pub fn close(&mut self) {
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::{prelude::*, JsCast};
            let document = web_sys::window().unwrap().document().unwrap();
            document
                .get_element_by_id("keyboard_trigger")
                .unwrap()
                .dyn_into::<web_sys::HtmlElement>()
                .unwrap()
                .blur()
                .unwrap();
            self.open = false;
        }
    }
}

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
pub(crate) struct TouchAdapter {
    pub primary: Option<Finger>,
    pub tracked: HashMap<Finger, Click>,
}

impl TouchAdapter {
    pub fn new() -> Self {
        Self {
            primary: None,
            tracked: HashMap::new(),
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
pub(crate) struct MouseAdapter {
    pub location: Option<Position<DeviceView>>,
    pub button_cache: HashMap<MouseButton, ElementState>,
    pub clicks: HashMap<MouseButton, Click>,
}

pub type ElementStateExpt = ElementState;
pub type MouseButtonExpt = MouseButton;

impl MouseAdapter {
    pub(crate) fn new() -> Self {
        Self {
            location: None,
            button_cache: HashMap::new(),
            clicks: HashMap::new(),
        }
    }
}

#[derive(Resource, Copy, Clone)]
pub enum Orientation {
    Portrait(f32),
    Landscape(f32),
}

impl Orientation {
    pub fn new<A: Into<Area<DeviceView>>>(window_size: A) -> Self {
        let window_size = window_size.into();
        let aspect_ratio = window_size.width / window_size.height;
        match aspect_ratio >= 1.0 {
            true => Landscape(aspect_ratio),
            false => Portrait(aspect_ratio),
        }
    }
}

#[derive(Resource, Clone, Copy)]
pub struct ScaleFactor {
    pub(crate) factor: f64,
}

impl ScaleFactor {
    pub(crate) fn new(factor: f64) -> Self {
        Self { factor }
    }
}

impl From<f64> for ScaleFactor {
    fn from(value: f64) -> Self {
        Self::new(value)
    }
}

#[derive(Clone, Copy)]
pub struct Resize {
    pub size: Area<DeviceView>,
    pub scale_factor: f64,
}

impl Resize {
    pub(crate) fn new(size: Area<DeviceView>, scale_factor: f64) -> Self {
        Self { size, scale_factor }
    }
}

pub struct WindowPlugin;

impl Attach for WindowPlugin {
    fn attach(engen: &mut Engen) {
        engen
            .frontend
            .container
            .insert_resource(Events::<Resize>::default());
        engen
            .backend
            .container
            .insert_resource(Events::<Resize>::default());
        engen
            .frontend
            .container
            .insert_resource(Events::<ClickEvent>::default());
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::First, Events::<Resize>::update_system);
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::First, Events::<ClickEvent>::update_system);
        engen
            .backend
            .main
            .add_system_to_stage(BackendStages::Initialize, Events::<Resize>::update_system);
        engen
            .frontend
            .container
            .insert_resource(TouchAdapter::new());
        engen
            .frontend
            .container
            .insert_resource(MouseAdapter::new());
        engen
            .frontend
            .container
            .insert_resource(VirtualKeyboardAdapter::new());
    }
}
