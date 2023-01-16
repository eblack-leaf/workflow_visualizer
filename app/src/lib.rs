use engen::{CanvasOptions, Engen, Task, Theme};

pub fn task() -> Task {
    let mut task = Task::new();
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
    engen.launch();
}
