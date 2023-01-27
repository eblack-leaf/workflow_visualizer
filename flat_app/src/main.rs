struct App;

impl flat_engen::FrontEnd for App {
    fn setup(task: &mut flat_engen::Task) {}
}

fn main() {
    let engen_descriptor = flat_engen::EngenDescriptor::new()
        .with_canvas_options(flat_engen::CanvasOptions::default())
        .with_theme(flat_engen::Theme::default())
        .with_native_dimensions(flat_engen::Area::new(700 as f32, 500 as f32))
        .with_min_canvas_dimensions(flat_engen::Area::new(400 as f32, 300 as f32));
    let mut engen = flat_engen::Engen::new(engen_descriptor);
    let args: Vec<String> = std::env::args().collect();
    #[cfg(not(target_arch = "wasm32"))]
    {
        if let Some(param) = args.get(1) {
            if param == "web-serve" {
                let compile_descriptor =
                    flat_engen::CompileDescriptor::new("flat_app", "--debug", "flat_app_web_build");
                engen
                    .compile_wasm_to(compile_descriptor).expect("could not compile")
                    .serve_at(([0, 0, 0, 0], 3030));
                return;
            }
        }
    }
    // engen.add_render_attachment::<flat_engen::TextRenderer>();
    engen.launch::<App>();
}
