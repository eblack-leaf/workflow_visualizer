use gloo_worker::{HandlerId, Registrable, Worker, WorkerBridge, WorkerScope};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;
use workflow_visualizer::winit::event_loop::{ControlFlow, EventLoopProxy};
use workflow_visualizer::{OutputWrapper, Receiver, Responder, Workflow};
use workflow_visualizer::{Visualizer, WorkflowWebExt};

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
    #[cfg(not(target_family = "wasm"))]
    pub async fn native_runner(responder: Responder<Response>, mut receiver: Receiver<Action>) {
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
    type Message = OutputWrapper<Engen>;
    type Input = Action;
    type Output = Response;

    fn create(scope: &WorkerScope<Self>) -> Self {
        Engen::new()
    }

    fn update(&mut self, scope: &WorkerScope<Self>, msg: Self::Message) {
        scope.respond(msg.handler_id, msg.response);
    }

    fn received(&mut self, scope: &WorkerScope<Self>, msg: Self::Input, id: HandlerId) {
        match msg {
            Action::ExitRequest => {
                scope.send_future(
                    async move { OutputWrapper::new(id, Self::Output::ExitConfirmed) },
                );
            }
            Action::AddToken(_) => {}
            Action::GenerateOtp(name) => scope.send_future(async move {
                OutputWrapper::new(
                    id,
                    Self::Output::TokenOtp((name, TokenOtp("88".to_string()))),
                )
            }),
            Action::RemoveToken(_) => {}
        }
    }
}
impl WorkflowWebExt for Engen {
    fn action_to_input(action: <Self as Workflow>::Action) -> <Self as Worker>::Input {
        return action;
    }
    fn output_to_response(output: <Self as Worker>::Output) -> <Self as Workflow>::Response {
        return output;
    }
}
fn main() {
    #[cfg(target_family = "wasm")]
    {
        console_error_panic_hook::set_once();
        Engen::registrar().register();
    }
}
