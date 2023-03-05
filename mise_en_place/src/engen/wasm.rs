#[cfg(target_arch = "wasm32")]
use std::rc::Rc;

#[cfg(target_arch = "wasm32")]
use winit::dpi::PhysicalSize;
#[cfg(target_arch = "wasm32")]
use winit::event_loop::EventLoop;
#[cfg(target_arch = "wasm32")]
use winit::window::{Window, WindowBuilder};

#[cfg(target_arch = "wasm32")]
use crate::Engen;
#[cfg(target_arch = "wasm32")]
use crate::engen::ignite::ignite;
#[cfg(target_arch = "wasm32")]
use crate::gfx::{GfxOptions, GfxSurface};

#[cfg(target_arch = "wasm32")]
pub(crate) async fn web_ignite(mut engen: Engen) {
    use wasm_bindgen::{JsCast, prelude::*};
    use winit::platform::web::WindowExtWebSys;
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init().expect("could not initialize logger");
    let event_loop = EventLoop::new();
    let window = Rc::new(
        WindowBuilder::new()
            .with_title("web engen")
            .build(&event_loop)
            .expect("could not create window"),
    );
    web_sys::window()
        .and_then(|win| win.document())
        .and_then(|doc| doc.body())
        .and_then(|body| {
            body.append_child(&web_sys::Element::from(window.canvas()))
                .ok()
        })
        .expect("couldn't append canvas to document body");
    engen.event_loop.replace(event_loop);
    engen.attach_scale_factor(window.scale_factor());
    window.set_inner_size(window_dimensions(window.scale_factor()));
    web_resizing(&window);
    let gfx = GfxSurface::new(&window, GfxOptions::web()).await;
    engen.backend.container.insert_resource(gfx.0);
    engen.backend.container.insert_resource(gfx.1);
    engen.window.replace(window);
    if let Err(error) = call_catch(&Closure::once_into_js(move || ignite(engen))) {
        let is_control_flow_exception = error.dyn_ref::<js_sys::Error>().map_or(false, |e| {
            e.message().includes("Using exceptions for control flow", 0)
        });
        if !is_control_flow_exception {
            web_sys::console::error_1(&error);
        }
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(catch, js_namespace = Function, js_name = "prototype.call.call")]
        fn call_catch(this: &JsValue) -> Result<(), JsValue>;
    }
}

#[cfg(target_arch = "wasm32")]
fn window_dimensions(scale_factor: f64) -> PhysicalSize<u32> {
    web_sys::window()
        .and_then(|win| win.document())
        .and_then(|doc| doc.body())
        .and_then(|body: web_sys::HtmlElement| -> Option<PhysicalSize<u32>> {
            let width: u32 = body.client_width().try_into().unwrap();
            let height: u32 = body.client_height().try_into().unwrap();
            Some(PhysicalSize::new(
                (width as f64 * scale_factor) as u32,
                (height as f64 * scale_factor) as u32,
            ))
        })
        .expect("could not create inner size")
}

#[cfg(target_arch = "wasm32")]
fn web_resizing(window: &Rc<Window>) {
    use wasm_bindgen::{JsCast, prelude::*};
    let w_window = window.clone();
    let closure = Closure::wrap(Box::new(move |_e: web_sys::Event| {
        let scale_factor = w_window.scale_factor();
        let size = window_dimensions(scale_factor);
        w_window.set_inner_size(size);
    }) as Box<dyn FnMut(_)>);
    let _ = web_sys::window()
        .expect("no web_sys window")
        .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref());
    match web_sys::window().expect("no web_sys window").screen() {
        Ok(screen) => {
            let _ = screen
                .orientation()
                .add_event_listener_with_callback("onchange", closure.as_ref().unchecked_ref());
        }
        Err(_) => {}
    }
    closure.forget();
}
