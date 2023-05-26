use crate::workflow::{Engen, TokenName};
use workflow_visualizer::bevy_ecs::prelude::{Local, Query, Res};
use workflow_visualizer::bevy_ecs::system::NonSend;
use workflow_visualizer::{Sender, Text, Workflow};

pub(crate) fn send_event(
    sender: NonSend<Sender<Engen>>,
    mut text: Query<(&mut Text)>,
    mut limiter: Local<bool>,
) {
    if !*limiter {
        let action = <Engen as Workflow>::Action::GenerateOtp(TokenName("not there".to_string()));
        sender.send(action);
        *limiter = true;
    }
    for mut t in text.iter_mut() {
        *t = Text("nooooo".to_string());
    }
}
