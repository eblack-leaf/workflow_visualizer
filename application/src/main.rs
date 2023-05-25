#![allow(unused, dead_code)]
mod system;
mod workflow;

use crate::workflow::Engen;
use workflow_visualizer::bevy_ecs::prelude::IntoSystemConfig;
#[cfg(target_os = "android")]
use workflow_visualizer::winit::platform::android::activity::AndroidApp;
use workflow_visualizer::{
    Area, Color, GfxOptions, Layer, Position, Request, Runner, Text, TextRequest,
    TextScaleAlignment, TextWrapStyle, Theme, ThemeDescriptor,
};
use workflow_visualizer::{
    Focus, FocusInputListener, TouchListener, Touchable, UserSpaceSyncPoint, Visualizer,
};

fn visualizer() -> Visualizer {
    let theme_desc = ThemeDescriptor::new().with_background(Color::DARK_CYAN);
    let mut visualizer = Visualizer::new(Theme::new(theme_desc), GfxOptions::native_defaults());
    visualizer
        .job
        .task(Visualizer::TASK_MAIN)
        .add_systems((system::send_event.in_set(UserSpaceSyncPoint::Process),));
    visualizer.add_entities(vec![Request::new(TextRequest::new(
        Position::new(10.0, 100.0),
        Area::new(100.0, 30.0),
        Layer::new(1.0),
        "hello",
        TextScaleAlignment::Large,
        Color::GREEN,
        TextWrapStyle::word(),
    ))]);
    visualizer.add_entities(vec![(
        Request::new(TextRequest::new(
            Position::new(10.0, 130.0),
            Area::new(100.0, 30.0),
            Layer::new(1.0),
            "world.",
            TextScaleAlignment::Large,
            Color::GREEN,
            TextWrapStyle::word(),
        )),
        Touchable::new(TouchListener::on_press()),
        Focus::new(),
        FocusInputListener {},
    )]);
    visualizer
}
#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(android_app: AndroidApp) {
    // android_logger::init_once(
    //     android_logger::Config::default().with_max_level(log::LevelFilter::Trace),
    // );
    tracing_subscriber::fmt().init();
    let mut visualizer = visualizer();
    visualizer.set_gfx_options(GfxOptions::limited_environment());
    let runner = Runner::new()
        .with_android_app(android_app)
        .native_run::<Engen, _, _>(visualizer, Engen::native_runner);
}
fn main() {
    #[cfg(not(target_family = "wasm"))]
    tracing_subscriber::fmt().init();
    let mut visualizer = visualizer();
    #[cfg(not(target_family = "wasm"))]
    Runner::new()
        .with_desktop_dimensions((400, 600))
        .native_run::<Engen, _, _>(visualizer, Engen::native_runner);
    #[cfg(target_family = "wasm")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init().expect("could not initialize logger");
        visualizer.set_gfx_options(GfxOptions::limited_environment());
        Runner::new().web_run::<Engen>(visualizer, "./worker.js".to_string());
    }
}
