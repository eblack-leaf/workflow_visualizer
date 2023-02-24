use mise_en_place::{WasmCompiler, WasmServer};

pub fn compile_and_serve() {
    let args: Vec<String> = std::env::args().collect();
    let wasm_compiler =
        WasmCompiler::new("mise_en_place_app", "debug", "mise_en_place_app_web_build");
    let wasm_server = ();
    if args.contains(&"build".to_string()) {
        wasm_compiler.compile().expect("could not compile wasm");
        if !args.contains(&"serve".to_string()) {
            return;
        }
    }
    if args.contains(&"serve".to_string()) {
        WasmServer::serve_at(
            "mise_en_place_app_web_build",
            "localhost".to_string(),
            ([0, 0, 0, 0], 3030),
        );
        return;
    }
}
