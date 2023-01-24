use engen::{
    Area, CanvasOptions, Color, Depth, Engen, Position, Task, Text, TextBundle, TextRenderer,
    TextScale, Theme,
};

pub fn task() -> Task {
    let mut task = Task::new();
    let _tester = task
        .container
        .spawn(TextBundle::new(
            Text::new("pepperminnnntttt!!! hey...".to_string()),
            Position::new(10f32, 10f32),
            Depth::new(0f32),
            Color::rgb(1.0, 1.0, 1.0),
            TextScale::new(45u32),
        ))
        .id();
    task
}

#[cfg_attr(
    target_os = "android",
    ndk_glue::main(backtrace = "on", logger(level = "debug"))
)]
pub fn launch() {
    let mut engen = Engen::new(task());
    engen.set_canvas_options(CanvasOptions::default());
    engen.set_theme(Theme::default());
    engen.attach::<TextRenderer>();
    engen.launch();
}
