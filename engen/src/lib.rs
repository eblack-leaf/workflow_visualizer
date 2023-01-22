use bevy_ecs::prelude::{Resource, SystemStage};
use winit::event::{Event, StartCause, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::Window;

pub use crate::canvas::{Canvas, CanvasOptions};
use crate::canvas::{CanvasWindow, update_visibility_cache};
pub(crate) use crate::canvas::Visibility;
pub use crate::color::Color;
pub use crate::coord::{Area, Depth, Panel, Position, Section};
use crate::render::{ExtractCalls, Render, RenderCalls};
use crate::task::{Stage, WorkloadId};
pub use crate::task::Task;
pub use crate::text::{Scale, Text, TextBundle, TextRenderer};
pub use crate::theme::Theme;

mod canvas;
#[allow(unused)]
mod text;
mod color;
#[allow(unused)]
mod coord;
mod icon;
mod render;
mod task;
mod theme;
mod uniform;

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
                    "visibility",
                    SystemStage::single(canvas::visibility),
                );
                compute.main.schedule.add_stage_after(Stage::Last, "update visibility cache", SystemStage::single(update_visibility_cache));
                compute
            },
            render: Task::new(),
            extract_calls: ExtractCalls::new(),
            render_calls: RenderCalls::new(),
        }
    }
    pub fn attach<Renderer: Render + Attach + Resource>(&mut self) {
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
            wasm_bindgen_futures::spawn_local(async {
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
                let options = self
                    .render
                    .container
                    .get_resource::<CanvasOptions>()
                    .expect("no canvas options attached");
                self.attach_canvas(Canvas::new(&window, options.clone().web_align()).await);
                self.attach_window(window);
                self.attach_event_loop(event_loop);
                self.run();
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
        event_loop.run(move |event, _event_loop_window_target, control_flow| {
            control_flow.set_poll();
            match event {
                Event::NewEvents(start_cause) => match start_cause {
                    StartCause::ResumeTimeReached { .. } => {}
                    StartCause::WaitCancelled { .. } => {}
                    StartCause::Poll => {}
                    StartCause::Init => {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let window = Window::new(_event_loop_window_target)
                                .expect("could not create window");
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
                        self.attach_viewport_bounds();
                        self.compute.exec(WorkloadId::Startup);
                        self.render.exec(WorkloadId::Startup);
                    }
                },
                Event::WindowEvent {
                    window_id: _window_id,
                    event,
                } => match event {
                    WindowEvent::Resized(physical_size) => {
                        self.render
                            .container
                            .get_resource_mut::<Canvas>()
                            .expect("no canvas attached")
                            .adjust(physical_size.width, physical_size.height);
                        self.attach_viewport_bounds();
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
                        self.render
                            .container
                            .get_resource_mut::<Canvas>()
                            .expect("no canvas attached")
                            .adjust(new_inner_size.width, new_inner_size.height);
                        self.attach_viewport_bounds();
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
                        self.attach_viewport_bounds();
                        self.compute.activate();
                        self.render.activate();
                    }
                }
                Event::MainEventsCleared => {
                    if self.compute.active() {
                        self.compute.exec(WorkloadId::Main);
                        self.get_window().request_redraw();
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

    fn attach_viewport_bounds(&mut self) {
        let bounds = self
            .render
            .container
            .get_resource::<Canvas>()
            .unwrap()
            .viewport
            .bounds();
        self.compute.container.insert_resource(bounds);
    }
}
