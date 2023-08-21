#![allow(unused, dead_code)]

use tracing::Level;

use application_logic::Engen;
use application_logic::EntryAttachment;
#[cfg(target_os = "android")]
use workflow_visualizer::winit::platform::android::activity::AndroidApp;
use workflow_visualizer::{BundledIcon, IconBitmap, IconBitmapRequest, Runner};
use workflow_visualizer::{Color, GfxOptions, Theme, ThemeDescriptor, Visualizer};

mod web_worker;

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(android_app: AndroidApp) {
    tracing_subscriber::fmt().with_max_level(Level::WARN).init();
    let mut visualizer = visualizer();
    visualizer.set_gfx_options(GfxOptions::limited_environment());
    Runner::new()
        .with_android_app(android_app)
        .native_run::<Engen>(visualizer);
}

fn visualizer() -> Visualizer {
    let theme_desc = ThemeDescriptor::new().with_background(Color::OFF_BLACK);
    let mut visualizer = Visualizer::new(
        Theme::new(theme_desc),
        GfxOptions::native_defaults().with_msaa(1),
    );
    visualizer.add_attachment::<EntryAttachment>();
    visualizer.spawn(IconBitmapRequest::from((
        "edit",
        IconBitmap::bundled(BundledIcon::Edit),
    )));
    visualizer.spawn(IconBitmapRequest::from((
        "add",
        IconBitmap::bundled(BundledIcon::Add),
    )));
    visualizer.spawn(IconBitmapRequest::from((
        "page_left",
        IconBitmap::bundled(BundledIcon::ArrowLeft),
    )));
    visualizer.spawn(IconBitmapRequest::from((
        "page_right",
        IconBitmap::bundled(BundledIcon::ArrowRight),
    )));
    visualizer.spawn(IconBitmapRequest::from((
        "run",
        IconBitmap::bundled(BundledIcon::Run),
    )));
    visualizer.spawn(IconBitmapRequest::from((
        "delete",
        IconBitmap::bundled(BundledIcon::Delete),
    )));
    visualizer.spawn(IconBitmapRequest::from((
        "generate",
        IconBitmap::bundled(BundledIcon::Generate),
    )));
    visualizer
}

fn main() {
    #[cfg(not(target_family = "wasm"))]
    tracing_subscriber::fmt().with_max_level(Level::WARN).init();
    let mut visualizer = visualizer();
    #[cfg(not(target_family = "wasm"))]
    Runner::new()
        .with_desktop_dimensions((400, 600))
        .native_run::<Engen>(visualizer);
    #[cfg(target_family = "wasm")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init().expect("could not initialize logger");
        visualizer.set_gfx_options(GfxOptions::limited_environment());
        Runner::new().web_run::<Engen>(visualizer, "./worker.js".to_string());
    }
}
