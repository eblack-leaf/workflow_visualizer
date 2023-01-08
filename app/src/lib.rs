use engene::{Engen, Task, CanvasOptions, TextRenderer};

pub fn task() -> Task {
    let task = Task::new();
    // ...
    task
}
#[cfg_attr(
    target_os = "android",
    ndk_glue::main(backtrace = "on", logger(level = "debug"))
)]
pub fn launch() {
    let mut engen = Engen::new(task());
    engen.set_canvas_options(CanvasOptions::default());
    engen.attach::<TextRenderer>();
    // ...
    engen.launch();
}
