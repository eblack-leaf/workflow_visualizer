use engen::{App, LaunchOptions, Launcher};

pub fn app() -> App {
    let app = App::new();
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
