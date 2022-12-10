#[cfg_attr(
    target_os = "android",
    ndk_glue::main(backtrace = "on", logger(level = "debug"))
)]
#[cfg(target_os = "android")]
pub fn main() {
    let visualizer = Visualizer::new();
    visualizer.launch(job());
}
