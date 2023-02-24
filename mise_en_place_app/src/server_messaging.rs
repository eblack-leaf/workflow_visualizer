use mise_en_place::MessageHandler;
use mise_en_place::StatusCodeExpt;

pub(crate) struct ServerMessageHandler {}

impl ServerMessageHandler {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl MessageHandler for ServerMessageHandler {
    #[cfg(not(target_arch = "wasm32"))]
    fn handle(&mut self, user: String, pass: String, message: String) -> (StatusCodeExpt, String) {
        (StatusCodeExpt::OK, String::from("message handled"))
    }
}
