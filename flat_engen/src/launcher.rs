use std::rc::Rc;

use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

use crate::Engen;
use crate::viewport::Viewport;

pub(crate) struct Launcher {
    pub(crate) event_loop: EventLoop<()>,
    pub(crate) window: Option<Rc<Window>>,
}

impl Launcher {
    fn new() -> Self {
        Self {
            event_loop: EventLoop::new(),
            window: None,
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn native(mut engen: Engen) {
        Launcher::new().launch(engen);
    }
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn web(mut engen: Engen) {
        wasm_bindgen_futures::spawn_local(Self::web_launch(engen));
    }
    #[cfg(target_arch = "wasm32")]
    async fn web_launch(mut engen: Engen) {
        use wasm_bindgen::prelude::*;
        use winit::platform::web::WindowExtWebSys;
        let mut launcher = Launcher::new();
        let inner_size = |scale_factor: f64| {
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
        };
        let window = Rc::new(
            WindowBuilder::new()
                .build(&event_loop)
                .expect("window builder failed"),
        );
        let scale_factor = window.scale_factor();
        window.set_inner_size(inner_size(scale_factor));
        {
            let w_window = window.clone();
            let closure = Closure::wrap(Box::new(move |e: web_sys::Event| {
                let scale_factor = w_window.scale_factor();
                let size = inner_size(scale_factor);
                w_window.set_inner_size(size);
            }) as Box<dyn FnMut(_)>);
            web_sys::window()
                .expect("no web_sys window")
                .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
                .unwrap();
            closure.forget();
        }
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| doc.body())
            .and_then(|body| {
                body.append_child(&web_sys::Element::from(window.canvas()))
                    .ok()
            })
            .expect("couldn't append canvas to document body");
        // TODO make and attach canvas in this async context
        launcher.window.replace(window);
        if let Err(error) = call_catch(&Closure::once_into_js(move || launcher.launch(engen))) {
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
    fn launch(mut self, mut engen: Engen) {
        engen.attach::<Viewport>();
        // ...
        self.event_loop.run(|| {});
    }
}
