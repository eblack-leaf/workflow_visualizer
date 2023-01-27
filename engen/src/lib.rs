#![allow(unused)]

use std::u32;

use bevy_ecs::prelude::{IntoSystemDescriptor, Resource, SystemStage};
use winit::dpi::PhysicalSize;
use winit::event::{Event, StartCause, WindowEvent};
use winit::event_loop::{EventLoop, EventLoopWindowTarget};
use winit::window::Window;

pub(crate) use visibility::Visibility;

pub use crate::canvas::{Canvas, CanvasOptions};
use crate::canvas::{CanvasWindow, Viewport};
pub use crate::color::Color;
use crate::coord::Scale;
pub use crate::coord::{Area, Depth, Panel, Position, Section};
use crate::render::{Extract, ExtractCalls, Render, RenderCalls};
pub use crate::task::Task;
use crate::task::{Stage, WorkloadId};
pub use crate::text::{Text, TextBundle, TextRenderer, TextScale};
pub use crate::theme::Theme;
use crate::visibility::{
    move_viewport_bounds, visibility, ViewportBounds, ViewportBoundsMovement, ViewportBoundsScale,
    VisibleEntities,
};

mod canvas;
mod color;
#[allow(unused)]
mod coord;
mod icon;
mod render;
mod task;
#[allow(unused)]
mod text;
mod theme;
mod uniform;
mod visibility;

pub trait Attach {
    fn attach(engen: &mut Engen);
}

pub struct Engen {
    event_loop: Option<EventLoop<()>>,
    pub compute: Task,
    render: Task,
    extract_calls: ExtractCalls,
    render_calls: RenderCalls,
}

impl Engen {
    pub fn new(mut compute: Task) -> Self {
        Self {
            event_loop: None,
            compute: {
                compute.main.schedule.add_stage_before(
                    Stage::After,
                    "move viewport bounds",
                    SystemStage::parallel()
                        .with_system(move_viewport_bounds.label("move viewport bounds"))
                        .with_system(visibility.after("move viewport bounds")),
                );
                compute
            },
            render: Task::new(),
            extract_calls: ExtractCalls::new(),
            render_calls: RenderCalls::new(),
        }
    }
    pub fn attach<Renderer: Render + Extract + Attach + Resource>(&mut self) {
        Renderer::attach(self);
        self.extract_calls.add(render::call_extract::<Renderer>);
        self.render_calls
            .add(Renderer::phase(), render::call_render::<Renderer>);
    }
    pub fn set_canvas_options(&mut self, options: CanvasOptions) {
        self.render.container.insert_resource(options);
    }
    pub fn set_theme(&mut self, theme: Theme) {
        self.render.container.insert_resource(theme);
    }
    fn attach_event_loop(&mut self, event_loop: EventLoop<()>) {
        self.event_loop.replace(event_loop);
    }
    fn attach_canvas(&mut self, canvas: Canvas) {
        self.render.container.insert_resource(canvas);
    }
    fn attach_window(&mut self, window: Window) {
        #[cfg(target_arch = "wasm32")]
        {
            self.render
                .container
                .insert_non_send_resource(CanvasWindow(window));
            return;
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.render.container.insert_resource(CanvasWindow(window));
        }
    }
    #[allow(unused)]
    fn detach_window(&mut self) -> Window {
        #[cfg(target_arch = "wasm32")]
        {
            return self
                .render
                .container
                .remove_non_send_resource::<CanvasWindow>()
                .expect("no canvas window attached")
                .0;
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.render
                .container
                .remove_resource::<CanvasWindow>()
                .expect("no canvas window attached")
                .0
        }
    }
    fn get_window(&self) -> &Window {
        #[cfg(target_arch = "wasm32")]
        {
            return &self
                .render
                .container
                .get_non_send_resource::<CanvasWindow>()
                .expect("no canvas window attached")
                .0;
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            return &self
                .render
                .container
                .get_non_send_resource::<CanvasWindow>()
                .expect("no canvas window attached")
                .0;
        }
    }
    pub fn launch(mut self) {
        #[cfg(target_arch = "wasm32")]
        {
            wasm_bindgen_futures::spawn_local(async move {
                std::panic::set_hook(Box::new(console_error_panic_hook::hook));
                console_log::init().expect("could not initialize logger");
                let event_loop = EventLoop::new();
                use winit::platform::web::WindowExtWebSys;
                let window = Window::new(&event_loop).expect("could not create window");
                // pulled from abstreet to add callback to resize browser viewport
                use wasm_bindgen::{prelude::*, JsCast};
                let get_full_size = || {
                    // TODO Not sure how to get scrollbar dims
                    let scrollbars = 30.0;
                    let win = web_sys::window().unwrap();
                    // `inner_width` corresponds to the browser's `self.innerWidth` function, which are in
                    // Logical, not Physical, pixels
                    winit::dpi::LogicalSize::new(
                        win.inner_width().unwrap().as_f64().unwrap() - scrollbars,
                        win.inner_height().unwrap().as_f64().unwrap() - scrollbars,
                    )
                };
                {
                    let winit_window = winit_window.clone();
                    let closure =
                        wasm_bindgen::closure::Closure::wrap(Box::new(move |e: web_sys::Event| {
                            debug!("handling resize event: {:?}", e);
                            let size = get_full_size();
                            winit_window.set_inner_size(size)
                        })
                            as Box<dyn FnMut(_)>);
                    web_sys_window
                        .add_event_listener_with_callback(
                            "resize",
                            closure.as_ref().unchecked_ref(),
                        )
                        .unwrap();
                    closure.forget();
                }
                // end abstreet code
                web_sys::window()
                    .and_then(|win| win.document())
                    .and_then(|doc| doc.body())
                    .and_then(|body| {
                        body.append_child(&web_sys::Element::from(window.canvas()))
                            .ok()
                    })
                    .expect("couldn't append canvas to document body");
                let options = self
                    .render
                    .container
                    .get_resource::<CanvasOptions>()
                    .expect("no canvas options attached");
                self.attach_canvas(Canvas::new(&window, options.clone().web_align()).await);
                self.attach_window(window);
                self.attach_event_loop(event_loop);
                if let Err(error) = call_catch(&Closure::once_into_js(move || self.run())) {
                    let is_control_flow_exception =
                        error.dyn_ref::<js_sys::Error>().map_or(false, |e| {
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
            });
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.attach_event_loop(EventLoop::new());
            self.run();
        }
    }
    fn run(mut self) {
        let event_loop = self.event_loop.take().expect("no event loop provided");
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
                            self.setup_native_window(event_loop_window_target);
                        }
                        self.set_canvas_size();
                        self.setup_viewport_bounds();
                        self.exec_startup();
                    }
                },
                Event::WindowEvent {
                    window_id: _window_id,
                    event,
                } => match event {
                    WindowEvent::Resized(physical_size) => {
                        self.adjust_canvas_size(physical_size);
                        self.adjust_viewport_bounds(physical_size);
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
                        scale_factor: _scale_factor,
                        new_inner_size,
                    } => {
                        self.adjust_canvas_size(*new_inner_size);
                        self.adjust_viewport_bounds(*new_inner_size);
                    }
                    WindowEvent::ThemeChanged(_) => {}
                    WindowEvent::Occluded(_) => {}
                },
                Event::DeviceEvent { .. } => {}
                Event::UserEvent(_) => {}
                Event::Suspended => {
                    if self.compute.active() {
                        #[cfg(target_os = "android")]
                        {
                            let _ = self.render.container.remove_resource::<Canvas>();
                        }
                        self.compute.suspend();
                        self.render.suspend();
                    }
                }
                Event::Resumed => {
                    if self.compute.suspended() {
                        #[cfg(target_os = "android")]
                        {
                            let window = self.detach_window();
                            let options = self
                                .render
                                .container
                                .get_resource::<CanvasOptions>()
                                .expect("no canvas options attached");
                            self.attach_canvas(futures::executor::block_on(Canvas::new(
                                &window,
                                options.clone(),
                            )));
                            self.attach_window(window);
                        }
                        self.compute.activate();
                        self.render.activate();
                    }
                }
                Event::MainEventsCleared => {
                    if self.compute.active() {
                        self.compute.exec(WorkloadId::Main);
                    }
                    if self.compute.should_exit() {
                        control_flow.set_exit();
                    }
                }
                Event::RedrawRequested(_window_id) => {
                    if self.render.active() {
                        render::extract(&mut self);
                        self.render.exec(WorkloadId::Main); // preparation for render
                        render::render(&mut self);
                    }
                    if self.render.should_exit() {
                        control_flow.set_exit();
                    }
                }
                Event::RedrawEventsCleared => {
                    if self.render.active() {
                        self.get_window().request_redraw();
                    }
                    if self.compute.can_idle() && self.render.can_idle() {
                        control_flow.set_wait();
                    }
                }
                Event::LoopDestroyed => {
                    self.compute.exec(WorkloadId::Teardown);
                    self.render.exec(WorkloadId::Teardown);
                }
            }
        });
    }
    #[cfg(not(target_arch = "wasm32"))]
    fn setup_native_window(&mut self, _event_loop_window_target: &EventLoopWindowTarget<()>) {
        let window = Window::new(_event_loop_window_target).expect("could not create window");
        let options = self
            .render
            .container
            .get_resource::<CanvasOptions>()
            .expect("no canvas options provided");
        self.attach_canvas(futures::executor::block_on(Canvas::new(
            &window,
            options.clone(),
        )));
        self.attach_window(window);
    }

    fn setup_viewport_bounds(&mut self) {
        self.compute
            .container
            .insert_resource(ViewportBounds::new(Section::new(
                (0.0, 0.0),
                (800.0, 600.0), // pull from viewport
            )));
        self.compute
            .container
            .insert_resource(VisibleEntities::new());
        self.compute
            .container
            .insert_resource(ViewportBoundsMovement::new());
        self.compute
            .container
            .insert_resource(ViewportBoundsScale::new());
        self.extract_calls
            .add(render::call_extract::<ViewportBounds>);
    }

    fn exec_startup(&mut self) {
        self.compute.exec(WorkloadId::Startup);
        self.render.exec(WorkloadId::Startup);
    }

    fn adjust_viewport_bounds(&mut self, physical_size: PhysicalSize<u32>) {
        self.compute
            .container
            .get_resource_mut::<ViewportBounds>()
            .expect("no bounds")
            .adjust(Scale::new(
                physical_size.width as f32,
                physical_size.height as f32,
            ));
    }

    fn adjust_canvas_size(&mut self, physical_size: PhysicalSize<u32>) {
        self.render
            .container
            .get_resource_mut::<Canvas>()
            .expect("no canvas attached")
            .adjust(physical_size.width, physical_size.height);
    }
    fn set_canvas_size(&mut self) {
        let new_size = self.set_window_inner();
        self.adjust_canvas_size(new_size);
        let aspect_ratio = new_size.width as f32 / new_size.height as f32;
        let orientation = match aspect_ratio > 1f32 {
            true => Orientation::Landscape,
            false => Orientation::Portrait,
        };
    }

    fn set_window_inner(&mut self) -> PhysicalSize<u32> {
        #[cfg(target_arch = "wasm32")]
        {
            let window = self
                .render
                .container
                .get_non_send_resource_mut::<CanvasWindow>()
                .expect("no canvas window");
            use winit::platform::web::WindowExtWebSys;
            let monitor_size = web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| doc.body())
                .and_then(|body: web_sys::HtmlElement| -> Option<PhysicalSize<u32>> {
                    let width: u32 = body.client_width().try_into().unwrap();
                    let height: u32 = body.client_height().try_into().unwrap();
                    Some(PhysicalSize::new(
                        (width as f64 * window.0.scale_factor()) as u32,
                        (height as f64 * window.0.scale_factor()) as u32,
                    ))
                })
                .expect("could not create body size");
            window.0.set_inner_size(monitor_size);
            monitor_size
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let window = self
                .render
                .container
                .get_resource_mut::<CanvasWindow>()
                .expect("no canvas window");
            let monitor_size = window.0.current_monitor().expect("no monitor info").size();
            window.0.set_inner_size(monitor_size);
            monitor_size
        }
    }
}

pub(crate) enum Orientation {
    Landscape,
    Portrait,
}
