use bevy_ecs::prelude::{IntoSystemDescriptor, Resource, SystemStage};
use std::rc::Rc;

use winit::dpi::PhysicalSize;
use winit::event::{Event, StartCause, WindowEvent};
use winit::event_loop::{EventLoop, EventLoopWindowTarget};
use winit::window::{Window, WindowBuilder};

use crate::canvas::Canvas;
use crate::coord::GpuArea;
use crate::extract::invoke_extract;
use crate::task::{Stage, WorkloadId};
use crate::viewport::{attach, Viewport};
use crate::visibility::{
    integrate_spacial_hash_changes, update_spacial_hash, visibility,
    ScaleFactor, Visibility, VisibleBounds,
};
use crate::{render, Engen, Section, Theme};

pub(crate) struct Launcher {
    pub(crate) event_loop: Option<EventLoop<()>>,
}

impl Launcher {
    fn new() -> Self {
        Self {
            event_loop: Some(EventLoop::new()),
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
        use wasm_bindgen::{prelude::*, JsCast};
        use winit::platform::web::WindowExtWebSys;
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init().expect("could not initialize logger");
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
                .build(launcher.event_loop.as_ref().expect("no event loop"))
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
        engen.window.replace(window);
        Canvas::attach(&mut engen).await;
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
        let event_loop = self.event_loop.take().expect("no event loop");
        event_loop.run(move |event, event_loop_window_target, control_flow| {
            control_flow.set_poll();
            match event {
                Event::NewEvents(start_cause) => match start_cause {
                    StartCause::ResumeTimeReached { .. } => {}
                    StartCause::WaitCancelled { .. } => {}
                    StartCause::Poll => {}
                    StartCause::Init => {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            self.attach_native_window(&mut engen, event_loop_window_target);
                        }
                        self.core_instrumentation(&mut engen);
                        engen.frontend.exec(WorkloadId::Startup);
                        engen.backend.exec(WorkloadId::Startup);
                    }
                },
                Event::WindowEvent {
                    window_id: _window_id,
                    event,
                } => match event {
                    WindowEvent::Resized(physical_size) => {
                        Self::resize_callback(&mut engen, physical_size);
                    }
                    WindowEvent::Moved(_) => {}
                    WindowEvent::CloseRequested => {
                        control_flow.set_exit();
                    }
                    WindowEvent::Destroyed => {}
                    WindowEvent::DroppedFile(_) => {}
                    WindowEvent::HoveredFile(_) => {}
                    WindowEvent::HoveredFileCancelled => {}
                    WindowEvent::ReceivedCharacter(_) => {}
                    WindowEvent::Focused(_) => {}
                    WindowEvent::KeyboardInput { .. } => {}
                    WindowEvent::ModifiersChanged(_) => {}
                    WindowEvent::Ime(_) => {}
                    WindowEvent::CursorMoved { .. } => {}
                    WindowEvent::CursorEntered { .. } => {}
                    WindowEvent::CursorLeft { .. } => {}
                    WindowEvent::MouseWheel { .. } => {}
                    WindowEvent::MouseInput { .. } => {}
                    WindowEvent::TouchpadPressure { .. } => {}
                    WindowEvent::AxisMotion { .. } => {}
                    WindowEvent::Touch(_) => {}
                    WindowEvent::ScaleFactorChanged {
                        scale_factor,
                        new_inner_size,
                    } => {
                        Self::resize_callback(&mut engen, *new_inner_size);
                    }
                    WindowEvent::ThemeChanged(_) => {}
                    WindowEvent::Occluded(_) => {}
                },
                Event::DeviceEvent { .. } => {}
                Event::UserEvent(_) => {}
                Event::Suspended => {
                    if engen.frontend.active() {
                        #[cfg(target_os = "android")]
                        {
                            let _ = self.backend.container.remove_resource::<Canvas>();
                        }
                        engen.frontend.suspend();
                        engen.backend.suspend();
                    }
                }
                Event::Resumed => {
                    if engen.frontend.suspended() {
                        #[cfg(target_os = "android")]
                        {
                            futures::executor::block_on(Canvas::attach(engen));
                        }
                        engen.frontend.activate();
                        engen.backend.activate();
                    }
                }
                Event::MainEventsCleared => {
                    if engen.frontend.active() {
                        engen.frontend.exec(WorkloadId::Main);
                    }
                    if engen.frontend.should_exit() {
                        control_flow.set_exit();
                    }
                }
                Event::RedrawRequested(_window_id) => {
                    if engen.backend.active() {
                        render::extract(&mut engen);
                        engen.backend.exec(WorkloadId::Main); // preparation for render
                        render::render(&mut engen);
                    }
                    if engen.backend.should_exit() {
                        control_flow.set_exit();
                    }
                }
                Event::RedrawEventsCleared => {
                    if engen.backend.active() {
                        engen.window.as_ref().expect("no window").request_redraw();
                    }
                    if engen.frontend.can_idle() && engen.backend.can_idle() {
                        control_flow.set_wait();
                    }
                }
                Event::LoopDestroyed => {
                    engen.frontend.exec(WorkloadId::Teardown);
                    engen.backend.exec(WorkloadId::Teardown);
                }
            }
        });
    }
    #[cfg(not(target_arch = "wasm32"))]
    fn attach_native_window(
        &self,
        mut engen: &mut Engen,
        event_loop_window_target: &EventLoopWindowTarget<()>,
    ) {
        let mut builder = WindowBuilder::new();
        if let Some(dimensions) = engen.engen_options.native_dimensions {
            builder =
                builder.with_inner_size(PhysicalSize::new(dimensions.width, dimensions.height));
        }
        let window = builder
            .build(event_loop_window_target)
            .expect("could not create window");
        engen.window.replace(Rc::new(window));
        futures::executor::block_on(Canvas::attach(&mut engen));
    }

    fn resize_callback(mut engen: &mut Engen, physical_size: PhysicalSize<u32>) {
        Canvas::get_mut(&mut engen).adjust(physical_size.width, physical_size.height);
        let canvas = engen
            .backend
            .container
            .remove_resource::<Canvas>()
            .expect("no canvas attached");
        engen
            .backend
            .container
            .get_resource_mut::<Viewport>()
            .expect("no viewport attached")
            .adjust_area(&canvas, physical_size.width, physical_size.height);
        engen
            .frontend
            .container
            .get_resource_mut::<VisibleBounds>()
            .expect("no visible bounds")
            .adjust().area = GpuArea::new(physical_size.width as f32, physical_size.height as f32)
            .as_area(engen.window.as_ref().expect("no window").scale_factor());
        engen.backend.container.insert_resource(canvas);
    }

    fn core_instrumentation(&self, engen: &mut Engen) {
        let window = engen.window.as_ref().expect("no window");
        engen
            .frontend
            .container
            .insert_resource(ScaleFactor::new(window.scale_factor()));
        engen
            .backend
            .container
            .insert_resource(ScaleFactor::new(window.scale_factor()));
        engen.frontend.main.schedule.add_stage_before(
            Stage::After,
            "update spacial hash",
            SystemStage::single(update_spacial_hash),
        );
        engen.frontend.main.schedule.add_stage_after(
            "update spacial hash",
            "integrate spacial hash",
            SystemStage::single(integrate_spacial_hash_changes),
        );
        engen.frontend.main.schedule.add_stage_after(
            "integrate spacial hash",
            "visibility",
            SystemStage::single(visibility),
        );
        let window_dimensions = window.inner_size();
        let window_dimensions = GpuArea::new(
            window_dimensions.width as f32,
            window_dimensions.height as f32,
        );
        let area = window_dimensions.as_area(window.scale_factor());
        engen
            .frontend
            .container
            .insert_resource(VisibleBounds::new(Section::new((0.0, 0.0), area)));
        engen
            .extract_fns
            .push(Box::new(invoke_extract::<VisibleBounds>));
        engen
            .backend
            .startup
            .schedule
            .add_system_to_stage(Stage::First, attach);
        engen
            .backend
            .container
            .insert_resource(engen.engen_options.theme.clone());
    }
}