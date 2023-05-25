use crate::workflow::{Engen, TokenName};
use workflow_visualizer::bevy_ecs::prelude::{Local, Query, Res};
use workflow_visualizer::bevy_ecs::system::NonSend;
use workflow_visualizer::{NativeSender, Text, WebSender, Workflow, WorkflowWebExt};

pub(crate) fn send_event(
    web_sender: NonSend<WebSender<Engen>>,
    native_sender: NonSend<NativeSender<Engen>>,
    mut text: Query<(&mut Text)>,
    mut limiter: Local<bool>,
) {
    if !*limiter {
        let action = <Engen as Workflow>::Action::GenerateOtp(TokenName("not there".to_string()));
        #[cfg(target_family = "wasm")]
        Engen::send(
            &web_sender,
            action,
        );
        #[cfg(not(target_family = "wasm"))]
        native_sender.send(action);
        *limiter = true;
    }
    for mut t in text.iter_mut() {
        *t = Text("nooooo".to_string());
    }
}
