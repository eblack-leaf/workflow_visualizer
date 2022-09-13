use winit::event_loop::EventLoopBuilder;

fn main() {
    let mut event_loop_builder = EventLoopBuilder::<app::WakeMessage>::with_user_event();
    let event_loop = event_loop_builder.build();
    let mut app = app::App::new();
    // create proxies and add to compute and render world
    futures::executor::block_on(app::run(app, event_loop));
}
