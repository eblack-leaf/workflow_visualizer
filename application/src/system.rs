use crate::workflow::{Engen, TokenName};
use workflow_visualizer::bevy_ecs::prelude::{Local, NonSend, Query, Res};
use workflow_visualizer::{Area, InterfaceContext, Position, Sender, Text, Workflow};

pub(crate) fn send_event(
    sender: NonSend<Sender<Engen>>,
    mut text: Query<(
        &mut Text,
        &Position<InterfaceContext>,
        &Area<InterfaceContext>,
    )>,
    mut limiter: Local<bool>,
) {
    if !*limiter {
        let action = <Engen as Workflow>::Action::GenerateOtp(TokenName("not there".to_string()));
        sender.send(action);
        *limiter = true;
    }
    for (mut t, pos, area) in text.iter_mut() {
        *t = Text(format!("pos: {:?}, area: {:?}", pos, area));
    }
}
