#![allow(unused, dead_code)]

use tracing::Level;

use application_logic::Engen;
use application_logic::EntryAttachment;
#[cfg(target_os = "android")]
use workflow_visualizer::winit::platform::android::activity::AndroidApp;
use workflow_visualizer::Runner;
use workflow_visualizer::{Color, GfxOptions, Theme, ThemeDescriptor, Visualizer};

mod web_worker;

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(android_app: AndroidApp) {
    tracing_subscriber::fmt().with_max_level(Level::WARN).init();
    let mut visualizer = visualizer();
    visualizer.set_gfx_options(GfxOptions::limited_environment());
    Runner::new()
        .with_attachment::<EntryAttachment>()
        .with_android_app(android_app)
        .native_run::<Engen>(visualizer);
}

fn visualizer() -> Visualizer {
    let theme_desc = ThemeDescriptor::new().with_background(Color::OFF_BLACK);
    let mut visualizer = Visualizer::new(
        Theme::new(theme_desc),
        GfxOptions::native_defaults().with_msaa(1),
    );
    visualizer
}

fn main() {
    #[cfg(not(target_family = "wasm"))]
    tracing_subscriber::fmt().with_max_level(Level::WARN).init();
    let mut visualizer = visualizer();
    let runner = Runner::new().with_attachment::<EntryAttachment>();
    #[cfg(not(target_family = "wasm"))]
    runner
        .with_desktop_dimensions((560, 900))
        .native_run::<Engen>(visualizer);
    #[cfg(target_family = "wasm")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init().expect("could not initialize logger");
        visualizer.set_gfx_options(GfxOptions::limited_environment());
        runner.web_run::<Engen>(visualizer, "./worker.js".to_string());
    }
}
