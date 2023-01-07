use engene::{Engen, Task};

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
    engen.launch();
}
