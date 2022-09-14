use winit::event_loop::EventLoopBuilder;

fn main() {
    let mut event_loop_builder = EventLoopBuilder::<app::WakeMessage>::with_user_event();
    let event_loop = event_loop_builder.build();
    let mut app = app::App::new();
    app.render
        .container
        .insert_resource(app::canvas::Options::defaults());
    // create proxies and add to compute and render world
    app::run(app, event_loop);
}
