#[cfg(target_family = "wasm")]
use crate::workflow::bridge::WebSender;
use crate::workflow::bridge::{add_exit_signal_handler, OutputWrapper};
use crate::workflow::run::internal_loop;
use crate::workflow::runner::EngenHandle;
use crate::{Runner, Sender, Visualizer, Workflow};
use gloo_worker::{HandlerId, Worker, WorkerScope};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use winit::dpi::PhysicalSize;
use winit::event_loop::EventLoopBuilder;
use winit::window::{Window, WindowBuilder};
impl<T: Workflow + Default + 'static> Worker for EngenHandle<T> {
    type Message = OutputWrapper<T>;
    type Input = T::Action;
    type Output = T::Response;

    fn create(scope: &WorkerScope<Self>) -> Self {
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
            body.append_child(&web_sys::Element::from(window.canvas()))
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
                (width as f64 * scale_factor) as u32,
                (height as f64 * scale_factor) as u32,
            ))
        })
        .expect("could not create inner size")
}

#[cfg(target_arch = "wasm32")]
fn web_resizing(window: &Rc<Window>) {
    use wasm_bindgen::{prelude::*, JsCast};
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
#[cfg(target_family = "wasm")]
pub(crate) async fn internal_web_run<T: Workflow + 'static + Default>(
    mut runner: Runner,
    mut visualizer: Visualizer,
    worker_path: String,
) {
    let event_loop = EventLoopBuilder::<T::Response>::with_user_event().build();
    let mut window = Some(Rc::new(
        WindowBuilder::new()
            .with_title("workflow_visualizer")
            .build(&event_loop)
            .expect("window"),
    ));
    add_web_canvas(window.as_ref().unwrap());
    window
        .as_ref()
        .unwrap()
        .set_inner_size(window_dimensions(window.as_ref().unwrap().scale_factor()));
    visualizer.init_gfx(window.as_ref().unwrap()).await;
    web_resizing(window.as_ref().unwrap());
    let proxy = event_loop.create_proxy();
    use gloo_worker::Spawnable;
    let bridge = EngenHandle::<T>::spawner()
        .callback(move |response| {
            proxy.send_event(response);
        })
        .spawn(worker_path.as_str());
    let bridge = Box::leak(Box::new(bridge));
    visualizer
        .job
        .container
        .insert_non_send_resource(Sender::new(WebSender(bridge)));
    let mut initialized = true;
    add_exit_signal_handler::<T>(&mut visualizer);
    use winit::platform::web::EventLoopExtWebSys;
    event_loop.spawn(move |event, event_loop_window_target, control_flow| {
        internal_loop::<T>(
            &mut visualizer,
            &mut window,
            &mut initialized,
            event,
            event_loop_window_target,
            control_flow,
            None,
        );
    });
}
