#![allow(unused, dead_code)]

use logic::Launcher;
use mise_en_place::{
    Engen, EngenOptions, IconPlugin, MessageReceiver, TextPlugin, WasmCompiler, WasmServer,
};

mod logic;
#[cfg(not(target_arch = "wasm32"))]
mod server;

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    { server::compile_and_serve(); }
    let mut engen = Engen::new(EngenOptions::new().with_native_dimensions((500, 900)));
    engen.add_plugin::<TextPlugin>();
    engen.add_plugin::<IconPlugin>();
    engen.add_plugin::<MessageReceiver>();
    engen.launch::<Launcher>();
}
