use mise_en_place::{Engen, Job, Launch, WasmCompiler};

struct JobHandle;

impl Launch for JobHandle {
    fn setup(job: &mut Job) {}
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"web".to_string()) {
        let compiler =
            WasmCompiler::new("mise_en_place_app", "debug", "mise_en_place_app_web_build");
        let server = Engen::compile_wasm(compiler).expect("could not compile to wasm");
        #[cfg(not(target_arch = "wasm32"))]{
            if args.contains(&"serve".to_string()) {
                server.serve_at(([0, 0, 0, 0], 3030));
            }
        }
    }
    let engen = Engen::new();
    engen.launch::<JobHandle>();
}
