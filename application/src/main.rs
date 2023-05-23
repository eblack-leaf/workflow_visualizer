#![allow(unused, dead_code)]
mod workflow;
use crate::workflow::Engen;
#[cfg(target_os = "android")]
use workflow_visualizer::winit::platform::android::activity::AndroidApp;
use workflow_visualizer::{Area, Color, GfxOptions, Runner, Theme, ThemeDescriptor, Visualizer};
#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(android_app: AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Trace),
    );
    // tracing_subscriber::fmt().init();
    let theme_desc = ThemeDescriptor::new().with_background(Color::DARK_ORANGE);
    let mut visualizer = Visualizer::new(Theme::new(theme_desc), GfxOptions::limited_environment());
    let runner = Runner::<Engen>::new()
        .with_android_app(android_app)
        .native_run(visualizer, Engen::native_runner);
}
fn main() {
    #[cfg(not(target_family = "wasm"))]
    tracing_subscriber::fmt().init();
    #[cfg(target_family = "wasm")] {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init().expect("could not initialize logger");
    }
    let theme_desc = ThemeDescriptor::new().with_background(Color::DARK_ORANGE);
    let mut visualizer = Visualizer::new(Theme::new(theme_desc), GfxOptions::native_defaults());
    #[cfg(not(target_family = "wasm"))]
    Runner::<Engen>::new()
            .with_desktop_dimensions((400, 600))
            .native_run(visualizer, Engen::native_runner);
    #[cfg(target_family = "wasm")] {
        visualizer.set_gfx_options(GfxOptions::limited_environment());
        Runner::<Engen>::new().web_run(visualizer);
    }
}
