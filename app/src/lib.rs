mod custom_render_attachment;

use crate::custom_render_attachment::CustomRenderAttachment;
use r_engen::{CanvasOptions, Engen, Task, TextAttachment};

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
    engen.attach(TextAttachment::default());
    // ...
    engen.attach(CustomRenderAttachment::default());
    engen.launch();
}
