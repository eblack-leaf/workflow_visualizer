#[cfg(target_arch = "wasm32")]
async fn main() {
    let visualizer = Visualizer::new();
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init().expect("could not initialize logger");
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).expect("could not create window");
    use winit::platform::web::WindowExtWebSys;
    web_sys::window()
        .and_then(|win| win.document())
        .and_then(|doc| doc.body())
        .and_then(|body| {
            body.append_child(&web_sys::Element::from(window.canvas()))
                .ok()
        })
        .expect("couldn't append canvas to document body");
    visualizer.attach_canvas(visualizer.create_canvas(&window).await);
    visualizer.attach_window(window);
    visualizer.attach_event_loop(event_loop);
    visualizer.launch(job());
}
