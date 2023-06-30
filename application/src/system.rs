use workflow_visualizer::{Area, EntityStore, InterfaceContext, Position, ScaleFactor, Sender, TextValue, TouchTrigger, Workflow};
use workflow_visualizer::bevy_ecs::prelude::{Local, NonSend, Query, Res};

use crate::workflow::{Engen, TokenName};

pub(crate) fn send_event(
    sender: NonSend<Sender<Engen>>,
    mut text: Query<(
        &mut TextValue,
        &Position<InterfaceContext>,
        &Area<InterfaceContext>,
    )>,
    buttons: Query<(&TouchTrigger)>,
    entity_store: Res<EntityStore>,
    mut limiter: Local<bool>,
    scale_factor: Res<ScaleFactor>,
) {
    if !*limiter {
        let action = <Engen as Workflow>::Action::GenerateOtp(TokenName("not there".to_string()));
        sender.send(action);
        *limiter = true;
    }
    for (mut t, pos, area) in text.iter_mut() {}
    if let Some(btn) = entity_store.get("edit-button") {
        if let Ok(btn_trigger) = buttons.get(btn) {
            if btn_trigger.triggered() {
                let action = <Engen as Workflow>::Action::GenerateOtp(TokenName("editing token".to_string()));
                sender.send(action);
            }
        }
    }
}
