use std::collections::HashMap;

use bevy_ecs::prelude::{
    Bundle, Component, Entity, EventReader, Events, IntoSystemConfig, Query, Res, ResMut, Resource,
};
use tracing::{debug, trace};
use winit::event::{ElementState, MouseButton};

use crate::{
    Area, DeviceContext, InterfaceContext, Layer, Position, ScaleFactor, Section, SyncPoint,
};
use crate::focus::FocusedEntity;
use crate::viewport::ViewportHandle;
use crate::visualizer::{Attach, Visualizer};

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

/// Where a Touch took place
#[derive(Component, Clone)]
pub struct TouchLocation(pub Option<TrackedTouch>);

/// Whether received a touch internally
#[derive(Component, Copy, Clone)]
pub struct TouchTrigger {
    touched: bool,
}

/// Currently touched or not logically
#[derive(Component, Copy, Clone)]
pub struct CurrentlyPressed {
    currently_pressed: bool,
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
    toggle: bool,
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
pub(crate) fn read_touch_events(
    mut event_reader: EventReader<TouchEvent>,
    mut primary_touch: ResMut<PrimaryTouch>,
    mut touch_listeners: Query<(
        Entity,
        &Position<InterfaceContext>,
        &Area<InterfaceContext>,
        &Layer,
        &TouchListener,
        &mut TouchTrigger,
        &mut CurrentlyPressed,
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
                        trace!("setting current touch");
                        prime.current = touch.touch;
                    }
                    if let Some(entity) = focused_entity.entity {
                        if let Ok((_, _, _, _, _, _, _, _, mut touch_location)) =
                            touch_listeners.get_mut(entity)
                        {
                            if let Some(mut tracked) = touch_location.0 {
                                trace!("setting current touch");
                                tracked.current = touch.touch;
                            }
                        }
                    }
                }
                TouchType::OnRelease => {
                    if let Some(prime) = primary_touch.touch.as_mut() {
                        prime.end.replace(touch.touch);
                    }
                    if let Some(entity) = focused_entity.entity {
                        if let Ok((_, _, _, _, _, _, mut pressed, _, _)) =
                            touch_listeners.get_mut(entity)
                        {
                            pressed.currently_pressed = false;
                        }
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
                            let touch_origin = prime.origin.to_interface(scale_factor.factor())
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
                                let touch_origin = prime.origin.to_interface(scale_factor.factor())
                                    - viewport_handle.section.position;
                                let touch_end = end.to_interface(scale_factor.factor())
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
                mut touch_trigger,
                mut currently_pressed,
                mut toggle_state,
                mut touch_location,
            )) = touch_listeners.get_mut(grabbed.0)
            {
                if trigger_on_press {
                    currently_pressed.currently_pressed = true;
                    if let ListenableTouchType::OnPress = listener.listened_type {
                        touch_trigger.touched = true;
                        toggle_state.toggle = !toggle_state.toggle;
                        touch_location.0.replace(primary_touch.touch.unwrap());
                        focused_entity.entity.replace(grabbed.0);
                    }
                }
                if trigger_on_release {
                    currently_pressed.currently_pressed = false;
                    if let ListenableTouchType::OnRelease = listener.listened_type {
                        touch_trigger.touched = true;
                        toggle_state.toggle = !toggle_state.toggle;
                        touch_location.0.replace(primary_touch.touch.unwrap());
                        focused_entity.entity.replace(grabbed.0);
                    }
                }
            }
        } else if trigger_on_press && focused_entity.entity.is_some() {
            let _ = focused_entity.entity.take();
        }
    }
}

pub(crate) fn reset_touched(mut touch_listeners: Query<&mut TouchTrigger>) {
    for mut touched in touch_listeners.iter_mut() {
        touched.touched = false;
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
pub(crate) struct TouchAttachment;
impl Attach for TouchAttachment {
    fn attach(visualizer: &mut Visualizer) {
        visualizer
            .job
            .container
            .insert_resource(PrimaryTouch::new());
        visualizer
            .job
            .container
            .insert_resource(TouchGrabState::new());
        visualizer
            .job
            .container
            .insert_resource(Events::<TouchEvent>::default());
        visualizer
            .job
            .container
            .insert_resource(TouchAdapter::new());
        visualizer
            .job
            .container
            .insert_resource(MouseAdapter::new());
        visualizer.job.task(Visualizer::TASK_MAIN).add_systems((
            Events::<TouchEvent>::update_system.in_set(SyncPoint::Event),
            read_touch_events.in_set(SyncPoint::Preparation),
            reset_touched.in_set(SyncPoint::Finish),
        ));
    }
}
