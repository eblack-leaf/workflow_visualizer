use engen::text::{Scale, Text, TextBundle};
use engen::{text, CanvasOptions, Color, Depth, Engen, Position, Task, Theme, Visibility};

pub fn task() -> Task {
    let mut task = Task::new();
    let tester = task
        .container
        .spawn(TextBundle::new(
            Text::new("ah".to_string()),
            Scale::new(13u32),
            Position::new(10f32, 10f32),
            Depth::new(0f32),
            Color::rgb(1.0, 1.0, 1.0),
        ))
        .id();
    task.container
        .get_mut::<Visibility>(tester)
        .expect("")
        .visible = true;
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
    // ...
    engen.attach::<text::Renderer>();
    engen.launch();
}
