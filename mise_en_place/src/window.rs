use std::collections::HashMap;

use bevy_ecs::prelude::{Commands, Component, EventReader, Events, Res, ResMut, Resource};
use winit::event::{ElementState, MouseButton};

use crate::{Area, FrontEndStartupStages, Position, VisibleBounds};
use crate::coord::DeviceView;
use crate::engen::{Attach, Engen};
use crate::engen::{BackendStages, FrontEndStages};
use crate::window::Orientation::{Landscape, Portrait};

#[derive(Resource)]
pub struct VirtualKeyboardAdapter {}

#[derive(Component, Copy, Clone)]
pub enum VirtualKeyboardType {
    Keyboard,
    TelephonePad,
    NumberPad,
}

impl VirtualKeyboardAdapter {
    pub(crate) fn new() -> Self {
        #[cfg(target_arch = "wasm32")]
        {
            let document = web_sys::window().unwrap().document().unwrap();
            let node = document.create_element("div").unwrap();
            node.set_inner_html(
                "<input type='text' maxlength='0' width=0 height=0 \
            id='keyboard_trigger' style='position: absolute;left: -1000px;top: -1000px;opacity: 0;\
            padding: 0;min-width: 0; min-height: 0;width: 0; height: 0;border: 0'>\
            <input type='tel' maxlength='0' width=0 height=0 \
            id='telephone_pad_trigger' style='position: absolute;left: -1000px;top: -1000px;opacity: 0;\
            padding: 0;min-width: 0; min-height: 0;width: 0; height: 0;border: 0'>\
            <input type='number' maxlength='0' width=0 height=0 \
            id='numpad_trigger' style='position: absolute;left: -1000px;top: -1000px;opacity: 0;\
            padding: 0;min-width: 0; min-height: 0;width: 0; height: 0;border: 0'>",
            );
            let body = document.body().unwrap();
            body.append_child(&node).unwrap();
        }
        Self {}
    }
    #[allow(unused)]
    pub fn open(&self, ty: VirtualKeyboardType) {
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::{JsCast, prelude::*};
            let document = web_sys::window().unwrap().document().unwrap();
            let trigger_element = match ty {
                VirtualKeyboardType::Keyboard => document
                    .get_element_by_id("keyboard_trigger")
                    .unwrap()
                    .dyn_into::<web_sys::HtmlElement>()
                    .unwrap(),
                VirtualKeyboardType::TelephonePad => document
                    .get_element_by_id("telephone_pad_trigger")
                    .unwrap()
                    .dyn_into::<web_sys::HtmlElement>()
                    .unwrap(),
                VirtualKeyboardType::NumberPad => document
                    .get_element_by_id("numpad_trigger")
                    .unwrap()
                    .dyn_into::<web_sys::HtmlElement>()
                    .unwrap(),
            };
            trigger_element.blur().unwrap();
            trigger_element.focus().unwrap();
        }
    }
    pub fn close(&self) {
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;
            let document = web_sys::window().unwrap().document().unwrap();
            document
                .get_element_by_id("keyboard_trigger")
                .unwrap()
                .dyn_into::<web_sys::HtmlElement>()
                .unwrap()
                .blur()
                .unwrap();
            document
                .get_element_by_id("telephone_pad_trigger")
                .unwrap()
                .dyn_into::<web_sys::HtmlElement>()
                .unwrap()
                .blur()
                .unwrap();
            document
                .get_element_by_id("numpad_trigger")
                .unwrap()
                .dyn_into::<web_sys::HtmlElement>()
                .unwrap()
                .blur()
                .unwrap();
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

pub(crate) fn setup_orientation(
    mut cmd: Commands,
    visible_bounds: Res<VisibleBounds>,
    scale_factor: Res<ScaleFactor>,
) {
    cmd.insert_resource(Orientation::new(
        visible_bounds.section.area.to_device(scale_factor.factor),
    ));
}

pub(crate) fn calc_orientation(
    mut events: EventReader<Resize>,
    mut orientation: ResMut<Orientation>,
) {
    for event in events.iter() {
        *orientation = Orientation::new(event.size);
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

pub struct WindowAttachment;

impl Attach for WindowAttachment {
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
            .startup
            .add_system_to_stage(FrontEndStartupStages::Initialization, setup_orientation);
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::Prepare, calc_orientation);
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
