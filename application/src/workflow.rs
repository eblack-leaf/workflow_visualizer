use std::collections::HashMap;
use gloo_worker::{HandlerId, Worker, WorkerScope};
use tracing::info;
use workflow_visualizer::winit::event_loop::{ControlFlow, EventLoopProxy};
use workflow_visualizer::Visualizer;
use workflow_visualizer::{Receiver, Responder, Workflow};

#[derive(Debug, Clone, PartialEq)]
pub enum Response {
    ExitConfirmed,
    TokenAdded(TokenName),
    TokenRemoved(TokenName),
    TokenOtp((TokenName, TokenOtp)),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    ExitRequest,
    AddToken((TokenName, Token)),
    GenerateOtp(TokenName),
    RemoveToken(TokenName),
}
#[derive(Debug, Clone, PartialEq)]
pub struct TokenOtp(pub String);
#[derive(Debug, Clone, PartialEq)]
pub struct TokenName(pub String);
#[derive(Debug, Clone, PartialEq)]
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
    #[cfg(not(target_family = "wasm"))]
    pub(crate) async fn native_runner(
        responder: Responder<Response>,
        mut receiver: Receiver<Action>,
    ) {
        let _engen = Engen::new();
        loop {
            while let Some(action) = receiver.receive().await {
                let response = match action {
                    Action::ExitRequest => <Engen as Workflow>::exit_response(),
                    Action::AddToken((name, token)) => Response::TokenAdded(name),
                    Action::GenerateOtp(name) => {
                        let otp = "".to_string();
                        Response::TokenOtp((name, TokenOtp(otp)))
                    }
                    Action::RemoveToken(name) => Response::TokenRemoved(name),
                };
                responder.respond(response);
            }
        }
    }
}
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
}

impl Worker for Engen {
    type Message = ();
    type Input = ();
    type Output = ();

    fn create(scope: &WorkerScope<Self>) -> Self {
        todo!()
    }

    fn update(&mut self, scope: &WorkerScope<Self>, msg: Self::Message) {
        // receive self messages after completing sends
        // use scope.respond(...) here
    }

    fn received(&mut self, scope: &WorkerScope<Self>, msg: Self::Input, id: HandlerId) {
        // send future to self using gloo_net::Request to db http endpoints
    }
}
