use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use workflow_visualizer::{Visualizer, Workflow};

use crate::entry::{ReadOtp, ReceivedTokens};

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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Hash, Eq)]
pub struct TokenName(pub String);

impl<T: Into<String>> From<T> for TokenName {
    fn from(value: T) -> Self {
        TokenName(value.into())
    }
}
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
        let mut engen = Engen::new();
        // include from file to generate hashmap
        engen
            .tokens
            .insert("school".into(), Token("245215".to_string()));
        engen
            .tokens
            .insert("work".into(), Token("245215".to_string()));
        engen
            .tokens
            .insert("home".into(), Token("245215".to_string()));
        engen
            .tokens
            .insert("personal".into(), Token("245215".to_string()));
        engen
            .tokens
            .insert("bank".into(), Token("245215".to_string()));
        engen
            .tokens
            .insert("email".into(), Token("245215".to_string()));
        engen
            .tokens
            .insert("phone".into(), Token("245215".to_string()));
        engen
            .tokens
            .insert("phone1".into(), Token("245215".to_string()));
        engen
            .tokens
            .insert("phone2".into(), Token("245215".to_string()));
        engen
            .tokens
            .insert("phone3".into(), Token("245215".to_string()));
        engen
            .tokens
            .insert("phone4".into(), Token("245215".to_string()));
        engen
            .tokens
            .insert("phone5".into(), Token("245215".to_string()));
        engen
            .tokens
            .insert("phone6".into(), Token("245215".to_string()));
        engen
            .tokens
            .insert("phone7".into(), Token("245215".to_string()));
        engen
            .tokens
            .insert("phone8".into(), Token("245215".to_string()));
        engen
            .tokens
            .insert("phone9".into(), Token("245215".to_string()));
        engen
            .tokens
            .insert("phone10".into(), Token("245215".to_string()));
        engen
    }
}
#[async_trait]
impl Workflow for Engen {
    type Action = Action;
    type Response = Response;

    fn handle_response(visualizer: &mut Visualizer, response: Self::Response) {
        match response {
            Response::TokenAdded(_name) => {}
            Response::TokenRemoved(_name) => {}
            Response::TokenOtp((name, otp)) => {
                visualizer.job.container.send_event(ReadOtp(name, otp));
            }
            Response::RequestedTokenNames(tokens) => {
                visualizer.job.container.send_event(ReceivedTokens(tokens));
            }
            Response::ExitConfirmed => {}
        }
    }

    async fn handle_action(engen: Arc<Mutex<Self>>, action: Self::Action) -> Self::Response {
        match action {
            Action::AddToken((name, _token)) => Response::TokenAdded(name),
            Action::GenerateOtp(name) => {
                let otp = "143647".to_string();
                Response::TokenOtp((name, TokenOtp(otp)))
            }
            Action::RemoveToken(name) => Response::TokenRemoved(name),
            Action::RequestTokenNames => Response::RequestedTokenNames(
                engen.lock().unwrap().tokens.keys().cloned().collect(),
            ),
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
