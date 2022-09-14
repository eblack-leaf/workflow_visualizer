pub mod android {
    #[cfg_attr(
        target_os = "android",
        ndk_glue::main(backtrace = "on", logger(level = "debug"))
    )]
    #[cfg(target_os = "android")]
    pub fn main() {
        let mut event_loop_builder =
            winit::event_loop::EventLoopBuilder::<app::WakeMessage>::with_user_event();
        let event_loop = event_loop_builder.build();
        let mut app = app::App::new();
        app.set_canvas_options(app::canvas::Options::defaults());
        // create proxies and add to compute and render jobs
        app::run(app, event_loop);
    }
}
