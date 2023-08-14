use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use workflow_visualizer::{Visualizer, Workflow};
#[derive(Default)]
pub(crate) struct Engen {}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) enum Action {
    ExitRequested,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) enum Response {
    ExitConfirmed,
}
#[async_trait]
impl Workflow for Engen {
    type Action = Action;
    type Response = Response;

    fn handle_response(_visualizer: &mut Visualizer, response: Self::Response) {
        match response {
            Response::ExitConfirmed => {}
        }
    }

    async fn handle_action(_engen: Arc<Mutex<Self>>, action: Self::Action) -> Self::Response {
        match action {
            Action::ExitRequested => Response::ExitConfirmed,
        }
    }

    fn exit_action() -> Self::Action {
        Action::ExitRequested
    }

    fn is_exit_response(res: &Self::Response) -> bool {
        match res {
            Response::ExitConfirmed => true,
            // _ => false,
        }
    }
}
