use mise_en_place::StatusCodeExpt;
use mise_en_place::{Message, MessageHandler, MessageType};

pub(crate) struct ServerMessageHandler {}

impl ServerMessageHandler {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl MessageHandler for ServerMessageHandler {
    #[cfg(not(target_arch = "wasm32"))]
    fn handle(
        &mut self,
        user: String,
        pass: String,
        ty: MessageType,
        message: Message,
    ) -> (StatusCodeExpt, (MessageType, Message)) {
        println!("received type: {:?}, message: {:?}", ty, message);
        (StatusCodeExpt::OK, (0, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]))
    }
}
