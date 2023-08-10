mod attachment;

use crate::{DeviceContext, Position};
use bevy_ecs::prelude::{EventReader, Res, ResMut, Resource};
use std::collections::HashMap;
use winit::event::{MouseButton, TouchPhase};
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Interaction(pub u32);
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
        PrimaryMouseButton(Interaction(MouseButton::Left.into()))
    }
}
#[derive(Resource)]
pub struct PrimaryInteraction(pub Option<Interaction>);
pub(crate) fn update_interactions(
    mut primary: ResMut<PrimaryInteraction>,
    mut events: EventReader<InteractionEvent>,
    primary_mouse: Res<PrimaryMouseButton>,
    mut locations: ResMut<InteractionLocations>,
    mut phases: ResMut<InteractionPhases>,
) {
    for event in events.iter() {
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
                    .insert(event.interaction, InteractionLocation::new(event.location));
            }
            InteractionPhase::Moved | InteractionPhase::Cancelled => {
                if let Some(mut loc) = locations.0.get_mut(&event.interaction) {
                    loc.current = event.location;
                }
            }
            InteractionPhase::Ended => {
                if let Some(mut loc) = locations.0.get_mut(&event.interaction) {
                    loc.end.replace(event.location);
                }
            }
        }
    }
}
pub(crate) fn process_primary(primary: Res<PrimaryInteraction>) {}
#[derive(Resource)]
pub struct InteractionPhases(pub HashMap<Interaction, InteractionPhase>);
pub(crate) struct InteractionEvent {
    pub(crate) device: InteractionDevice,
    pub(crate) location: Position<DeviceContext>,
    pub(crate) phase: InteractionPhase,
    pub(crate) interaction: Interaction,
}
pub struct InteractionLocation {
    pub(crate) start: Position<DeviceContext>,
    pub(crate) current: Position<DeviceContext>,
    pub(crate) end: Option<Position<DeviceContext>>,
}
impl InteractionLocation {
    pub fn new(start: Position<DeviceContext>) -> Self {
        Self {
            start,
            current: start,
            end: None,
        }
    }
}
#[derive(Resource)]
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
