use std::sync::{Arc, Mutex};

use gloo_worker::{HandlerId, Worker, WorkerScope};

use crate::workflow::bridge::OutputWrapper;
#[cfg(target_family = "wasm")]
use crate::workflow::bridge::WebSender;
use crate::workflow::runner::EngenHandle;
#[cfg(target_family = "wasm")]
use crate::Runner;
#[cfg(target_family = "wasm")]
use crate::Sender;
#[cfg(target_family = "wasm")]
use crate::Visualizer;
use crate::Workflow;
#[cfg(target_family = "wasm")]
use std::rc::Rc;
#[cfg(target_family = "wasm")]
use wasm_bindgen::JsValue;
#[cfg(target_family = "wasm")]
use winit::dpi::PhysicalSize;
#[cfg(target_family = "wasm")]
use winit::event_loop::EventLoopBuilder;
#[cfg(target_family = "wasm")]
use winit::window::{Window, WindowBuilder};

impl<T: Workflow + Default + 'static> Worker for EngenHandle<T> {
    type Message = OutputWrapper<T>;
    type Input = T::Action;
    type Output = T::Response;

    fn create(_scope: &WorkerScope<Self>) -> Self {
        EngenHandle(Arc::new(Mutex::new(T::default())))
    }

    fn update(&mut self, scope: &WorkerScope<Self>, msg: Self::Message) {
        scope.respond(msg.handler_id, msg.response);
    }

    fn received(&mut self, scope: &WorkerScope<Self>, msg: Self::Input, id: HandlerId) {
        let arc = self.0.clone();
        scope.send_future(async move {
            let response = <T as Workflow>::handle_action(arc, msg).await;
            OutputWrapper::new(id, response)
        });
    }
}
/// spawn a web worker using the types blanket implemented `gloo_worker`
pub fn start_web_worker<T: Workflow + Default + 'static>() {
    #[cfg(target_family = "wasm")]
    {
        use gloo_worker::Registrable;
        console_error_panic_hook::set_once();
        EngenHandle::<T>::registrar().register();
    }
}
#[cfg(target_arch = "wasm32")]
fn add_web_canvas(window: &Window) {
    use winit::platform::web::WindowExtWebSys;
    web_sys::window()
        .and_then(|win| win.document())
        .and_then(|doc| doc.body())
        .and_then(|body| {
            body.append_child(&web_sys::Element::from(window.canvas().expect("canvas")))
                .ok()
        })
        .expect("couldn't append canvas to document body");
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
                ((width as f64 * scale_factor) as u32).max(4),
                ((height as f64 * scale_factor) as u32).max(4),
            ))
        })
        .expect("could not create inner size")
}

#[cfg(target_arch = "wasm32")]
fn web_resizing(window: &Rc<Window>) {
    use wasm_bindgen::prelude::*;
    let w_window = window.clone();
    let closure = Closure::wrap(Box::new(move |_e: web_sys::Event| {
        let scale_factor = w_window.scale_factor();
        let size = window_dimensions(scale_factor);
        let size = PhysicalSize::new(size.width.max(4), size.height.max(4));
        let _ = w_window.request_inner_size(size);
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
#[cfg(target_family = "wasm")]
pub(crate) async fn internal_web_run<T: Workflow + 'static + Default>(
    _runner: Runner,
    mut visualizer: Visualizer,
    worker_path: String,
) {
    let event_loop = EventLoopBuilder::<T::Response>::with_user_event()
        .build()
        .expect("event-loop");
    let mut window = Some(Rc::new(
        WindowBuilder::new()
            .with_title("workflow_visualizer")
            .build(&event_loop)
            .expect("window"),
    ));
    add_web_canvas(window.as_ref().unwrap());
    let current_size = window
        .as_ref()
        .unwrap()
        .request_inner_size(PhysicalSize::new(4, 4));
    web_sys::console::info_1(&JsValue::from_str(
        format!("current-size: {:?}", current_size).as_str(),
    ));
    visualizer.init_gfx(window.as_ref().unwrap()).await;
    web_resizing(window.as_ref().unwrap());
    let proxy = event_loop.create_proxy();
    use gloo_worker::Spawnable;
    let bridge = EngenHandle::<T>::spawner()
        .callback(move |response| {
            let _ = proxy.send_event(response);
        })
        .spawn(worker_path.as_str());
    let bridge = Box::leak(Box::new(bridge));
    visualizer
        .job
        .container
        .insert_non_send_resource(Sender::new(WebSender(bridge)));
    let mut initialized = true;
    use winit::platform::web::EventLoopExtWebSys;
    let _ = event_loop.spawn(move |event, event_loop_window_target| {
        crate::workflow::run::internal_loop::<T>(
            &mut visualizer,
            &mut window,
            &mut initialized,
            event,
            event_loop_window_target,
            None,
        );
    });
}
