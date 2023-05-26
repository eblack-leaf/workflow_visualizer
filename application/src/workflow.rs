use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::info;
use workflow_visualizer::{async_trait, Runner, Workflow};
use workflow_visualizer::{Visualizer};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Response {
    ExitConfirmed,
    TokenAdded(TokenName),
    TokenRemoved(TokenName),
    TokenOtp((TokenName, TokenOtp)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Action {
    ExitRequest,
    AddToken((TokenName, Token)),
    GenerateOtp(TokenName),
    RemoveToken(TokenName),
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TokenOtp(pub String);
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TokenName(pub String);
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Token(pub String);
pub struct Engen {
    pub tokens: HashMap<TokenName, Token>,
}
impl Engen {
    pub fn new() -> Self {
        Self {
            tokens: HashMap::new(),
        }
    }
}
impl Default for Engen {
    fn default() -> Self {
        Engen::new()
    }
}
#[async_trait]
impl Workflow for Engen {
    type Action = Action;
    type Response = Response;

    fn handle_response(visualizer: &mut Visualizer, response: Self::Response) {
        match response {
            Response::ExitConfirmed => {
                // save?
            }
            Response::TokenAdded(name) => {
                info!("token added: {:?}", name);
            }
            Response::TokenRemoved(name) => {
                info!("token removed: {:?}", name);
            }
            Response::TokenOtp((name, otp)) => {
                info!("token otp: {:?}:{:?}", name, otp);
            }
        }
    }

    fn exit_action() -> Self::Action {
        Action::ExitRequest
    }

    fn exit_response() -> Self::Response {
        Response::ExitConfirmed
    }

    async fn handle_action(engen: Arc<Mutex<Self>>, action: Self::Action) -> Self::Response {
        match action {
            Action::ExitRequest => <Engen as Workflow>::exit_response(),
            Action::AddToken((name, token)) => Response::TokenAdded(name),
            Action::GenerateOtp(name) => {
                let otp = "".to_string();
                Response::TokenOtp((name, TokenOtp(otp)))
            }
            Action::RemoveToken(name) => Response::TokenRemoved(name),
        }
    }
}
fn main() {
    Runner::start_web_worker::<Engen>();
}
