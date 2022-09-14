use winit::event_loop::EventLoop;
use winit::window::Window;

fn main() {
    #[cfg(target_arch = "wasm32")]
    wasm_bindgen_futures::spawn_local(web_run());
}
#[cfg(target_arch = "wasm32")]
async fn web_run() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init().expect("could not initialize logger");
    let event_loop = EventLoop::new();
    let mut app = app::App::new();
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
    let options = app::canvas::Options::web_defaults();
    app.set_canvas_options(options.clone());
    app.attach_canvas(app::canvas::canvas(&window, options).await);
    app.attach_window(window);
    app::run(app, event_loop);
}
