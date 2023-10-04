use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use workflow_visualizer::{Visualizer, Workflow};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Response {
    ExitConfirmed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Action {
    ExitRequest,
}
pub struct Engen {}
impl Engen {
    pub fn new() -> Self {
        Self {}
    }
}
impl Default for Engen {
    fn default() -> Self {
        let engen = Engen::new();
        engen
    }
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
            Action::ExitRequest => Response::ExitConfirmed,
        }
    }

    fn exit_action() -> Self::Action {
        Action::ExitRequest
    }

    fn is_exit_response(res: &Self::Response) -> bool {
        match res {
            Response::ExitConfirmed => true,
        }
    }
}
