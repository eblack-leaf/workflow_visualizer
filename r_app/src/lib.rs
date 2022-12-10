use focal::{Job, Visualizer, VisualizerOptions};

pub mod android {
    #[cfg_attr(
        target_os = "android",
        ndk_glue::main(backtrace = "on", logger(level = "debug"))
    )]
    #[cfg(target_os = "android")]
    pub fn main() {
        crate::native();
    }
}
pub fn job() -> Job {
    let job = Job::new();
    job
}
pub fn native() {
    let visualizer = Visualizer::new(Some(VisualizerOptions::native()));
    visualizer.launch(job());
}
#[cfg(target_arch = "wasm32")]
pub async fn web() {
    let visualizer = Visualizer::web(Some(VisualizerOptions::web())).await;
    visualizer.launch(job());
}
