use std::collections::HashMap;

use bevy_ecs::bundle::Bundle;
use bevy_ecs::change_detection::{Res, ResMut};
use bevy_ecs::component::Component;
use bevy_ecs::event::EventReader;
use bevy_ecs::prelude::{Entity, Query, Resource, Without};
use winit::event::{ElementState, MouseButton, TouchPhase};

use crate::{
    Area, DeviceContext, Disabled, InterfaceContext, Layer, Position, ScaleFactor,
    Section, ViewportHandle,
};
use crate::focus::FocusedEntity;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct Interaction(pub u32);

impl From<u64> for Interaction {
    fn from(value: u64) -> Self {
        Interaction(value as u32)
    }
}

pub enum InteractionDevice {
    Mouse,
    Touchscreen,
}

impl From<MouseButton> for Interaction {
    fn from(value: MouseButton) -> Self {
        let i = match value {
            MouseButton::Left => 0,
            MouseButton::Right => 1,
            MouseButton::Middle => 2,
            MouseButton::Other(val) => 3 + val as u32,
        };
        Interaction(i)
    }
}

#[derive(Resource)]
pub struct PrimaryMouseButton(pub(crate) Interaction);

impl Default for PrimaryMouseButton {
    fn default() -> Self {
        PrimaryMouseButton(MouseButton::Left.into())
    }
}

#[derive(Resource, Default)]
pub struct PrimaryInteraction(pub Option<Interaction>);

pub(crate) fn update_interactions(
    mut primary: ResMut<PrimaryInteraction>,
    mut events: EventReader<InteractionEvent>,
    primary_mouse: Res<PrimaryMouseButton>,
    mut locations: ResMut<InteractionLocations>,
    mut phases: ResMut<InteractionPhases>,
    viewport_handle: Res<ViewportHandle>,
    scale_factor: Res<ScaleFactor>,
) {
    for event in events.iter() {
        let offset_location =
            event.location.to_interface(scale_factor.factor()) + viewport_handle.section.position;
        match event.device {
            InteractionDevice::Mouse => {
                if event.interaction == primary_mouse.0 {
                    if primary.0.is_none() {
                        if event.phase == InteractionPhase::Started {
                            primary.0.replace(event.interaction);
                        }
                    }
                }
            }
            InteractionDevice::Touchscreen => {
                if primary.0.is_none() {
                    if event.phase == InteractionPhase::Started {
                        primary.0.replace(event.interaction);
                    }
                }
            }
        }
        phases.0.insert(event.interaction, event.phase);
        match event.phase {
            InteractionPhase::Started => {
                locations
                    .0
                    .insert(event.interaction, InteractionLocation::new(offset_location));
            }
            InteractionPhase::Moved | InteractionPhase::Cancelled => {
                if let Some(mut loc) = locations.0.get_mut(&event.interaction) {
                    loc.current = offset_location;
                }
            }
            InteractionPhase::Ended => {
                if let Some(mut loc) = locations.0.get_mut(&event.interaction) {
                    loc.end.replace(offset_location);
                }
            }
        }
    }
}

#[derive(Bundle, Clone, Default)]
pub struct Interactable {
    pub triggered: Triggered,
    pub toggled: Toggled,
    pub tracker: InteractionTracker,
    pub active: ActiveInteraction,
}

#[derive(Component, Default, Copy, Clone)]
pub struct Triggered(pub(crate) bool);

impl Triggered {
    pub fn active(&self) -> bool {
        self.0
    }
}

#[derive(Component, Copy, Clone, Default)]
pub struct Toggled(pub(crate) bool);

impl Toggled {
    pub fn active(&self) -> bool {
        self.0
    }
}

#[derive(Component, Copy, Clone, Default)]
pub struct ActiveInteraction(pub(crate) bool);

impl ActiveInteraction {
    pub fn active(&self) -> bool {
        self.0
    }
}

#[derive(Component, Default, Clone)]
pub struct InteractionTracker {
    pub location: Option<InteractionLocation>,
}

pub(crate) struct InteractionGrab(pub(crate) Option<(Entity, Layer)>);

pub(crate) fn resolve(
    mut primary: ResMut<PrimaryInteraction>,
    mut interactable_entities: Query<
        (
            Entity,
            &Position<InterfaceContext>,
            &Area<InterfaceContext>,
            &Layer,
            &mut InteractionTracker,
            &mut Triggered,
            &mut Toggled,
            &mut ActiveInteraction,
        ),
        Without<Disabled>,
    >,
    locations: ResMut<InteractionLocations>,
    phases: Res<InteractionPhases>,
    mut focused_entity: ResMut<FocusedEntity>,
) {
    if let Some(prime) = primary.0 {
        let mut grab = InteractionGrab(None);
        let phase = *phases.0.get(&prime).expect("phase");
        if phase == InteractionPhase::Ended || phase == InteractionPhase::Cancelled {
            primary.0.take();
        }
        let location = locations.0.get(&prime).expect("location");
        for (entity, pos, area, layer, _, _, _, _) in interactable_entities.iter_mut() {
            match phase {
                InteractionPhase::Started => {
                    let section = Section::new(*pos, *area);
                    if section.contains(location.start) {
                        if let Some(grabbed) = grab.0.as_mut() {
                            if *layer < grabbed.1 {
                                grabbed.0 = entity;
                                grabbed.1 = *layer;
                            }
                        } else {
                            grab.0.replace((entity, *layer));
                        }
                    }
                }
                InteractionPhase::Moved => {}
                InteractionPhase::Ended => {}
                InteractionPhase::Cancelled => {}
            }
        }
        if let Some(grabbed) = grab.0.take() {
            if let Some(focused) = focused_entity.entity {
                if focused != grabbed.0 {
                    if let Ok((_, _, _, _, mut tracker, _, _, mut active)) =
                        interactable_entities.get_mut(focused)
                    {
                        tracker.location.take();
                        active.0 = false;
                    }
                }
            }
            focused_entity.entity.replace(grabbed.0);
        } else {
            if phase == InteractionPhase::Started {
                if let Some(ent) = focused_entity.entity.take() {
                    if let Ok((_, _, _, _, mut tracker, _, _, mut active)) =
                        interactable_entities.get_mut(ent)
                    {
                        active.0 = false;
                        tracker.location.take();
                    }
                }
            }
        }
        if let Some(focused) = focused_entity.entity {
            if let Ok((_, pos, area, _, mut tracker, mut triggered, mut toggled, mut active)) =
                interactable_entities.get_mut(focused)
            {
                match phase {
                    InteractionPhase::Started => {
                        tracker
                            .location
                            .replace(InteractionLocation::new(location.start));
                        active.0 = true;
                    }
                    InteractionPhase::Moved => {
                        if let Some(tracked) = tracker.location.as_mut() {
                            tracked.current = location.current;
                        }
                    }
                    InteractionPhase::Ended => {
                        let end = location.end.expect("end");
                        active.0 = false;
                        let section = Section::new(*pos, *area);
                        if section.contains(end) {
                            if let Some(tracked) = tracker.location.as_mut() {
                                triggered.0 = true;
                                toggled.0 = !toggled.0;
                                tracked.end.replace(end);
                            }
                        } else {
                            tracker.location.take();
                        }
                    }
                    InteractionPhase::Cancelled => {
                        tracker.location.take();
                        active.0 = false;
                    }
                }
            }
        }
    }
}

pub(crate) fn cleanup(
    mut trackers: Query<(
        Entity,
        &mut InteractionTracker,
        &mut Triggered,
        &mut ActiveInteraction,
        Option<&Disabled>,
    )>,
    mut focused_entity: ResMut<FocusedEntity>,
) {
    for (entity, mut tracker, mut triggered, mut active, disabled) in trackers.iter_mut() {
        if triggered.0 {
            triggered.0 = false;
        }
        if tracker.location.is_some() {
            if tracker.location.unwrap().end.is_some() || disabled.is_some() {
                tracker.location.take();
                if let Some(focused) = focused_entity.entity {
                    if entity == focused {
                        focused_entity.entity.take();
                    }
                }
            }
        } else {
            if active.active() {
                active.0 = false;
            }
        }
    }
}

#[derive(Resource, Default)]
pub struct InteractionPhases(pub HashMap<Interaction, InteractionPhase>);

pub struct InteractionEvent {
    pub(crate) device: InteractionDevice,
    pub(crate) location: Position<DeviceContext>,
    pub(crate) phase: InteractionPhase,
    pub(crate) interaction: Interaction,
}

impl InteractionEvent {
    pub fn new(
        device: InteractionDevice,
        location: Position<DeviceContext>,
        phase: InteractionPhase,
        interaction: Interaction,
    ) -> Self {
        Self {
            device,
            location,
            phase,
            interaction,
        }
    }
}

#[derive(Copy, Clone)]
pub struct InteractionLocation {
    pub(crate) start: Position<InterfaceContext>,
    pub(crate) current: Position<InterfaceContext>,
    pub(crate) end: Option<Position<InterfaceContext>>,
}

impl InteractionLocation {
    pub fn new(start: Position<InterfaceContext>) -> Self {
        Self {
            start,
            current: start,
            end: None,
        }
    }
}

#[derive(Resource, Default)]
pub struct InteractionLocations(pub HashMap<Interaction, InteractionLocation>);

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum InteractionPhase {
    Started,
    Moved,
    Ended,
    Cancelled,
}

impl From<winit::event::TouchPhase> for InteractionPhase {
    fn from(value: TouchPhase) -> Self {
        match value {
            TouchPhase::Started => InteractionPhase::Started,
            TouchPhase::Moved => InteractionPhase::Moved,
            TouchPhase::Ended => InteractionPhase::Ended,
            TouchPhase::Cancelled => InteractionPhase::Cancelled,
        }
    }
}

#[derive(Resource, Default)]
pub(crate) struct MouseAdapter {
    pub(crate) location: Position<DeviceContext>,
    pub(crate) button_cache: HashMap<MouseButton, ElementState>,
}

impl MouseAdapter {
    pub(crate) fn set_location<P: Into<Position<DeviceContext>>>(&mut self, location: P) {
        self.location = location.into();
    }
    pub(crate) fn cache_invalid(&mut self, button: MouseButton, value: ElementState) -> bool {
        if let Some(old) = self.button_cache.insert(button, value) {
            return if old != value { true } else { false };
        }
        true
    }
}
