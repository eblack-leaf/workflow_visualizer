use std::collections::HashMap;

use bevy_ecs::prelude::{
    Bundle, Component, Entity, EventReader, Events, IntoSystemConfig, Query, Res, ResMut, Resource,
};
use winit::event::{ElementState, MouseButton};

use crate::focus::FocusedEntity;
use crate::viewport::ViewportHandle;
use crate::visualizer::{Attach, Visualizer};
use crate::{
    Area, DeviceContext, InterfaceContext, Layer, Position, ScaleFactor, Section, SyncPoint,
};

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
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum TouchType {
    OnPress,
    OnMove,
    OnRelease,
    Cancelled,
}
pub type Touch = Position<DeviceContext>;
#[derive(Bundle, Copy, Clone)]
pub struct Touchable {
    pub(crate) touched: Touched,
    pub(crate) touched_state: TouchedState,
    pub(crate) toggle_state: ToggleState,
    pub(crate) listener: TouchListener,
    pub(crate) touch_location: TouchLocation,
}
impl Touchable {
    pub fn new(listener: TouchListener) -> Self {
        Self {
            touched: Touched::new(),
            touched_state: TouchedState::new(),
            toggle_state: ToggleState::new(),
            listener,
            touch_location: TouchLocation(None),
        }
    }
}
#[derive(Component, Copy, Clone)]
pub struct TouchListener {
    pub listened_type: ListenableTouchType,
}
impl TouchListener {
    pub fn on_press() -> Self {
        Self {
            listened_type: ListenableTouchType::OnPress,
        }
    }
    #[allow(unused)]
    pub fn on_release() -> Self {
        Self {
            listened_type: ListenableTouchType::OnRelease,
        }
    }
}
#[derive(Component, Copy, Clone, PartialOrd, PartialEq)]
pub struct TouchLocation(pub Option<Position<DeviceContext>>);
#[derive(Component, Copy, Clone)]
pub struct Touched {
    pub(crate) touched: bool,
}
#[derive(Component, Copy, Clone)]
pub struct TouchedState {
    pub currently_pressed: bool,
}
impl TouchedState {
    pub fn new() -> Self {
        Self {
            currently_pressed: false,
        }
    }
}
#[derive(Component, Copy, Clone)]
pub struct ToggleState {
    pub toggle: bool,
}
impl ToggleState {
    pub fn new() -> Self {
        Self { toggle: false }
    }
}
impl Touched {
    pub(crate) fn new() -> Self {
        Self { touched: false }
    }
    pub fn touched(&self) -> bool {
        self.touched
    }
}
#[allow(unused)]
#[derive(Copy, Clone)]
pub enum ListenableTouchType {
    OnPress,
    OnRelease,
}
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
#[derive(Resource)]
pub struct PrimaryTouch {
    pub touch: Option<TrackedTouch>,
}
impl PrimaryTouch {
    pub(crate) fn new() -> Self {
        Self { touch: None }
    }
}
#[derive(Resource)]
pub struct TouchGrabState {
    pub(crate) grab_state: Option<(Entity, Layer)>,
}
impl TouchGrabState {
    pub(crate) fn new() -> Self {
        Self { grab_state: None }
    }
}
pub(crate) fn read_touch_events(
    mut event_reader: EventReader<TouchEvent>,
    mut primary_touch: ResMut<PrimaryTouch>,
    mut touch_listeners: Query<(
        Entity,
        &Position<InterfaceContext>,
        &Area<InterfaceContext>,
        &Layer,
        &TouchListener,
        &mut Touched,
        &mut TouchedState,
        &mut ToggleState,
        &mut TouchLocation,
    )>,
    scale_factor: Res<ScaleFactor>,
    viewport_handle: Res<ViewportHandle>,
    mut touch_grab_state: ResMut<TouchGrabState>,
    mut focused_entity: ResMut<FocusedEntity>,
) {
    let new_touches = event_reader.iter().cloned().collect::<Vec<TouchEvent>>();
    let mut cancelled_events = new_touches.clone();
    cancelled_events.retain(|c| c.ty == TouchType::Cancelled);
    let is_cancelled = !cancelled_events.is_empty();
    let mut trigger_on_press = false;
    let mut trigger_on_release = false;
    if !is_cancelled {
        for touch in new_touches.iter() {
            match touch.ty {
                TouchType::OnPress => {
                    primary_touch.touch.replace(TrackedTouch::new(touch.touch));
                    trigger_on_press = true;
                }
                TouchType::OnMove => {
                    if let Some(prime) = primary_touch.touch.as_mut() {
                        prime.current = touch.touch;
                    }
                }
                TouchType::OnRelease => {
                    if let Some(prime) = primary_touch.touch.as_mut() {
                        prime.end.replace(touch.touch);
                    }
                    trigger_on_release = true;
                }
                _ => {}
            }
        }
    }
    if !new_touches.is_empty() && !is_cancelled {
        for (entity, pos, area, layer, listener, _, _, _, _) in touch_listeners.iter() {
            let section = Section::from((*pos, *area));
            match listener.listened_type {
                ListenableTouchType::OnPress => {
                    if trigger_on_press {
                        if let Some(prime) = primary_touch.touch {
                            let touch_origin = prime.origin.to_ui(scale_factor.factor)
                                - viewport_handle.section.position;
                            if section.contains(touch_origin) {
                                if let Some(grab_state) = touch_grab_state.grab_state.as_mut() {
                                    if *layer > grab_state.1 {
                                        grab_state.0 = entity;
                                        grab_state.1 = *layer;
                                    }
                                } else {
                                    touch_grab_state.grab_state.replace((entity, *layer));
                                }
                            }
                        }
                    }
                }
                ListenableTouchType::OnRelease => {
                    if trigger_on_release {
                        if let Some(prime) = primary_touch.touch {
                            if let Some(end) = prime.end {
                                let touch_origin = prime.origin.to_ui(scale_factor.factor)
                                    - viewport_handle.section.position;
                                let touch_end = end.to_ui(scale_factor.factor)
                                    - viewport_handle.section.position;
                                if section.contains(touch_origin) && section.contains(touch_end) {
                                    if let Some(grab_state) = touch_grab_state.grab_state.as_mut() {
                                        if *layer > grab_state.1 {
                                            grab_state.0 = entity;
                                            grab_state.1 = *layer;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        if let Some(grabbed) = touch_grab_state.grab_state.take() {
            if let Ok((
                _,
                _,
                _,
                _,
                listener,
                mut touched,
                mut touched_state,
                mut toggle_state,
                mut touch_location,
            )) = touch_listeners.get_mut(grabbed.0)
            {
                if trigger_on_press {
                    touched_state.currently_pressed = true;
                    if let ListenableTouchType::OnPress = listener.listened_type {
                            touched.touched = true;
                            toggle_state.toggle = !toggle_state.toggle;
                            touch_location
                                .0
                                .replace(primary_touch.touch.unwrap().origin);
                            focused_entity.entity.replace(grabbed.0);
                    }
                }
                if trigger_on_release {
                    touched_state.currently_pressed = false;
                    if let ListenableTouchType::OnRelease = listener.listened_type {
                        touched.touched = true;
                        toggle_state.toggle = !toggle_state.toggle;
                        touch_location
                            .0
                            .replace(primary_touch.touch.unwrap().origin);
                        focused_entity.entity.replace(grabbed.0);
                    }
                }
            }
        } else if trigger_on_press && focused_entity.entity.is_some() {
            let _ = focused_entity.entity.take();
        }
    }
}
pub(crate) fn reset_touched(mut touch_listeners: Query<&mut Touched>) {
    for mut touched in touch_listeners.iter_mut() {
        touched.touched = false;
    }
}
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
}
pub type CursorLocation = Position<DeviceContext>;
#[derive(Resource)]
pub struct MouseAdapter {
    pub location: Option<CursorLocation>,
    pub tracked: HashMap<MouseButton, (ElementState, Option<TrackedTouch>)>,
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
    fn attach(engen: &mut Visualizer) {
        engen.job.container.insert_resource(PrimaryTouch::new());
        engen.job.container.insert_resource(TouchGrabState::new());
        engen
            .job
            .container
            .insert_resource(Events::<TouchEvent>::default());
        engen.job.container.insert_resource(TouchAdapter::new());
        engen.job.container.insert_resource(MouseAdapter::new());
        engen.job.task(Visualizer::TASK_MAIN).add_systems((
            Events::<TouchEvent>::update_system.in_set(SyncPoint::Event),
            read_touch_events.in_set(SyncPoint::Config),
            reset_touched.in_set(SyncPoint::Finish),
        ));
    }
}
