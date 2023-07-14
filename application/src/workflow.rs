use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tracing::info;

use workflow_visualizer::{start_web_worker, Visualizer, Workflow};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Response {
    ExitConfirmed,
    TokenAdded(TokenName),
    TokenRemoved(TokenName),
    TokenOtp((TokenName, TokenOtp)),
    RequestedTokenNames(Vec<TokenName>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Action {
    ExitRequest,
    AddToken((TokenName, Token)),
    GenerateOtp(TokenName),
    RemoveToken(TokenName),
    RequestTokenNames,
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
            tokens: HashMap::new(), // include from file to generate hashmap
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

    fn handle_response(_visualizer: &mut Visualizer, response: Self::Response) {
        match response {
            Response::TokenAdded(name) => {
                info!("token added: {:?}", name);
            }
            Response::TokenRemoved(name) => {
                info!("token removed: {:?}", name);
            }
            Response::TokenOtp((name, otp)) => {
                info!("token otp: {:?}:{:?}", name, otp);
            }
            Response::RequestedTokenNames(tokens) => {
                // put into visualizer
            }
            Response::ExitConfirmed => {}
        }
    }

    async fn handle_action(_engen: Arc<Mutex<Self>>, action: Self::Action) -> Self::Response {
        match action {
            Action::AddToken((name, _token)) => Response::TokenAdded(name),
            Action::GenerateOtp(name) => {
                let otp = "".to_string();
                Response::TokenOtp((name, TokenOtp(otp)))
            }
            Action::RemoveToken(name) => Response::TokenRemoved(name),
            Action::RequestTokenNames => {
                // get tokens from engen
                Response::RequestedTokenNames(vec![])
            }
            Action::ExitRequest => Response::ExitConfirmed,
        }
    }

    fn exit_action() -> Self::Action {
        Action::ExitRequest
    }

    fn is_exit_response(res: &Self::Response) -> bool {
        match res {
            Response::ExitConfirmed => true,
            _ => false,
        }
    }
}
fn main() {
    start_web_worker::<Engen>();
}
