use flat_engen::{Color, Depth, Position, Text, TextBundle, TextScaleAlignment};

struct App;

impl flat_engen::FrontEnd for App {
    fn setup(task: &mut flat_engen::Task) {
        task.container.spawn(TextBundle::new(
            Text::new("Hello...".to_string()),
            Position::new(10.0, 10.0),
            Depth::new(0u32),
            Color::rgb(1.0, 1.0, 1.0),
            TextScaleAlignment::Medium,
        ));
    }
}

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let args: Vec<String> = std::env::args().collect();
        if let Some(param) = args.get(1) {
            if param.starts_with("web") {
                let compile_descriptor =
                    flat_engen::CompileDescriptor::new("flat_app", "--debug", "flat_app_web_build");
                let server = flat_engen::Engen::compile_wasm_to(compile_descriptor)
                    .expect("could not compile");
                if param == "web-compile" {
                    return;
                }
                if param == "web-serve" {
                    server.serve_at(([0, 0, 0, 0], 3030));
                    return;
                }
            }
        }
    }
    let engen_descriptor = flat_engen::EngenDescriptor::new()
        .with_canvas_options(flat_engen::CanvasOptions::default())
        .with_theme(flat_engen::Theme::default())
        .with_native_dimensions(flat_engen::Area::new(400 as f32, 600 as f32))
        .with_min_canvas_dimensions(flat_engen::Area::new(400 as f32, 300 as f32));
    let mut engen = flat_engen::Engen::new(engen_descriptor);
    engen.add_render_attachment::<flat_engen::TextRenderer>();
    engen.launch::<App>();
}
