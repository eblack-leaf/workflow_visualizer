use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use workflow_visualizer::{Visualizer, Workflow};

#[derive(Default)]
pub struct Engen {}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Action {
    ExitRequest,
    NoOp,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Response {
    ExitConfirmed,
    NoOp,
}
#[async_trait]
impl Workflow for Engen {
    type Action = Action;
    type Response = Response;

    fn handle_response(_visualizer: &mut Visualizer, _response: Self::Response) {}

    async fn handle_action(_engen: Arc<Mutex<Self>>, action: Self::Action) -> Self::Response {
        match action {
            Action::ExitRequest => Response::ExitConfirmed,
            Action::NoOp => Response::NoOp,
        }
    }

    fn exit_action() -> Self::Action {
        Action::ExitRequest
    }

    fn is_exit_response(res: &Self::Response) -> bool {
        match res {
            Response::ExitConfirmed => true,
            Response::NoOp => false,
        }
    }
}
