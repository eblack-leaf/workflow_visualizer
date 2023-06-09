use workflow_visualizer::{Area, InterfaceContext, Position, Sender, TextValue, Workflow};
use workflow_visualizer::bevy_ecs::prelude::{Local, NonSend, Query, Res};

use crate::workflow::{Engen, TokenName};

pub(crate) fn send_event(
    sender: NonSend<Sender<Engen>>,
    mut text: Query<(
        &mut TextValue,
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
        *t = TextValue(format!("{:?}, {:?}", pos, area));
    }
}
