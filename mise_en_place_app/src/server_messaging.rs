use mise_en_place::{resolve_message, to_message, MessageRepr, StatusCodeExpt};
use mise_en_place::{Message, MessageHandler, MessageType};

use crate::logic::IntMessage;

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
        if ty == IntMessage::message_type() {
            let mut resolved = resolve_message::<IntMessage>(message).unwrap();
            resolved.0 += 5;
            let encoded = resolved.to_message();
            if let Some(mes) = encoded {
                return (StatusCodeExpt::OK, (IntMessage::message_type(), mes));
            }
        }
        (StatusCodeExpt::OK, (1, vec![]))
    }
}
