struct App;
impl flat_engen::FrontEnd for App {
    fn setup(task: &mut flat_engen::Task) {
        todo!()
    }
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if let Some(param) = args.first() {
        if param == "--web" {
            let compile_descriptor =
                flat_engen::CompileDescriptor::new("flat_app", "--debug", "flat_app_web_build");
            flat_engen::Engen::compile_wasm_to(compile_descriptor).serve_at(([0, 0, 0, 0], 3030));
        }
    }
    let engen_descriptor = flat_engen::EngenDescriptor::new()
        .with_canvas_options(flat_engen::CanvasOptions::default())
        .with_theme(flat_engen::Theme::default());
    let mut engen = flat_engen::Engen::new(engen_descriptor);
    // engen.attach_renderer::<flat_engen::TextRenderer>();
    engen.launch::<App>();
}
