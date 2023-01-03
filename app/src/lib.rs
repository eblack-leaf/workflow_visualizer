use engen::{LaunchOptions, Launcher, Task};

pub fn app() -> Task {
    let app = Task::new();
    // ...
    app
}
#[cfg_attr(
    target_os = "android",
    ndk_glue::main(backtrace = "on", logger(level = "debug"))
)]
pub fn launch() {
    let launcher = Launcher::new(app(), LaunchOptions::default());
    launcher.launch();
}
