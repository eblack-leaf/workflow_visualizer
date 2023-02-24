use mise_en_place::{MessageRepr, resolve_message, StatusCodeExpt, to_message, WasmCompiler, WasmServer};
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

pub fn compile_and_serve() {
    let args: Vec<String> = std::env::args().collect();
    let wasm_compiler =
        WasmCompiler::new("mise_en_place_app", "debug", "mise_en_place_app_web_build");
    let wasm_server = WasmServer::new(&wasm_compiler);
    if args.contains(&"build".to_string()) {
        wasm_compiler.compile().expect("could not compile wasm");
        if !args.contains(&"serve".to_string()) {
            return;
        }
    }
    if args.contains(&"serve".to_string()) {
        wasm_server.serve_at(
            ([0, 0, 0, 0], 3030),
            ServerMessageHandler::new(),
        );
        return;
    }
}
