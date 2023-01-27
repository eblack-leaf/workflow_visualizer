use crate::Engen;
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

pub(crate) struct Launcher {
    pub(crate) event_loop: EventLoop<()>,
    pub(crate) window: Option<Window>,
}
impl Launcher {
    fn new() -> Self {
        Self {
            event_loop: EventLoop::new(),
            window: None,
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn native(engen: Engen) {
        Launcher::new().launch(engen);
    }
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn web(engen: Engen) {
        wasm_bindgen_futures::spawn_local(Self::web_launch(engen));
    }
    #[cfg(target_arch = "wasm32")]
    async fn web_launch(engen: Engen) {
        use wasm_bindgen::prelude::*;
        let mut launcher = Launcher::new();
        let window = WindowBuilder::new()
            .with_inner_size()
            .build(&event_loop)
            .expect("window builder failed");
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
    fn launch(mut self, engen: Engen) {
        self.event_loop.run(|| {});
    }
}
