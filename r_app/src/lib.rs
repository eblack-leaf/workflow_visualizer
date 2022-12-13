use focal::{Job, Gfx};

pub fn job() -> Job {
    let job = Job::new();
    job
}
#[cfg_attr(
target_os = "android",
ndk_glue::main(backtrace = "on", logger(level = "debug"))
)]
pub fn launch() {
    let gfx = Gfx::new();
    gfx.launch(job());
}
