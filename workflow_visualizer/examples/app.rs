use winit::event_loop::EventLoop;
use workflow_visualizer::{Engen, EngenOptions, Job, Launch};

#[cfg(not(target_arch = "wasm32"))]
pub fn compile_and_serve() {
    use workflow_visualizer::{WasmCompiler, WasmServer};
    let args: Vec<String> = std::env::args().collect();
    let wasm_compiler = WasmCompiler::new(
        "workflow_visualizer",
        "--example",
        "app",
        "release",
        "app_web_build",
    );
    if args.contains(&"build".to_string()) {
        wasm_compiler.compile().expect("could not compile wasm");
        if !args.contains(&"serve".to_string()) {
            return;
        }
    }
    if args.contains(&"serve".to_string()) {
        WasmServer::serve_at(
            "app_web_build",
            ([0, 0, 0, 0], 3030),
        );
        return;
    }
}
struct Launcher;
impl Launch for Launcher {
    fn options() -> EngenOptions {
        EngenOptions::new().with_native_dimensions((500, 900))
    }

    fn preparation(frontend: &mut Job) {
    }
}
fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    compile_and_serve();
    Engen::launch::<Launcher>(EventLoop::new());
}