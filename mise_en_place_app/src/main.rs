#![allow(unused, dead_code)]

use launcher::Launcher;
use mise_en_place::{Engen, EngenOptions, IconPlugin, MessageReceiver, TextPlugin, WasmCompiler, WasmServer};

#[cfg(not(target_arch = "wasm32"))]
mod server_messaging;
mod launcher;
mod logic;

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
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
            wasm_server.serve_at(([0, 0, 0, 0], 3030), server_messaging::ServerMessageHandler::new());
            return;
        }
    }
    let mut engen = Engen::new(EngenOptions::new().with_native_dimensions((500, 900)));
    engen.add_plugin::<TextPlugin>();
    engen.add_plugin::<IconPlugin>();
    engen.add_plugin::<MessageReceiver>();
    engen.launch::<Launcher>();
}
