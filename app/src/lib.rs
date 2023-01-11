use engen::{CanvasOptions, Color, Depth, Engen, Position, Scale, Task, Text, TextBundle, TextRenderer, Theme};

pub fn task() -> Task {
    let mut task = Task::new();
    task.container.spawn(TextBundle::new(
        Text::new("a".to_string()),
        Scale::new(60f32),
        Position::new(10f32, 10f32),
        Depth::new(0f32),
        Color::default(),
    ));
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
    // ...
    engen.launch();
}
