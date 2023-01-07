#![allow(unused)]
use crate::canvas::CanvasWindow;
pub use crate::canvas::Viewport;
pub use crate::canvas::{Canvas, CanvasOptions};
use crate::render::RenderPhases;
pub use crate::render::{Extract, Id, Render, RenderAttachment, RenderPassHandle, RenderPhase};
use crate::task::WorkloadId;
pub use crate::text::TextAttachment;
pub use task::Task;
use winit::event::{Event, StartCause, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::Window;

mod canvas;
mod color;
mod coord;
mod instance;
mod render;
mod task;
mod text;
mod theme;
mod uniform;

pub struct Engen {
    event_loop: Option<EventLoop<()>>,
    compute: Task,
    render: Task,
    extractors: Vec<Box<dyn Extract>>,
    render_attachments: Vec<Box<dyn RenderAttachment>>,
}
impl Engen {
    pub fn new(task: Task) -> Self {
        Self {
            event_loop: None,
            compute: task,
            render: Task::new(),
            extractors: Vec::new(),
            render_attachments: Vec::new(),
        }
    }
    fn attach_render_attachments(&mut self) {
        let mut attachments = self
            .render_attachments
            .drain(..)
            .collect::<Vec<Box<dyn RenderAttachment>>>();
        for attachment in attachments.drain(..) {
            self.extractors.push(attachment.extractor());
            let renderer = attachment.renderer(
                self.render
                    .container
                    .get_resource::<Canvas>()
                    .expect("no canvas attached"),
            );
            self.render
                .container
                .get_non_send_resource_mut::<RenderPhases>()
                .expect("no render phases attached")
                .insert(renderer);
            attachment.attach(self);
        }
    }
    pub fn attach<Attachment: RenderAttachment + 'static>(&mut self, attachment: Attachment) {
        self.render_attachments.push(Box::new(attachment));
    }
    pub fn set_canvas_options(&mut self, options: CanvasOptions) {
        self.render.container.insert_resource(options);
    }
    fn attach_event_loop(&mut self, event_loop: EventLoop<()>) {
        self.event_loop.replace(event_loop);
    }
    fn attach_canvas(&mut self, canvas: Canvas) {
        self.render.container.insert_resource(canvas);
    }
    fn attach_window(&mut self, window: Window) {
        self.render.container.insert_resource(CanvasWindow(window));
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
    fn extract(&mut self) {
        for extractor in self.extractors.iter_mut() {
            extractor.extract(&self.compute, &mut self.render);
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
                            self.attach_render_attachments();
                            self.attach_window(window);
                        }
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
                            let window = self
                                .render
                                .container
                                .remove_resource::<CanvasWindow>()
                                .expect("no canvas window attached");
                            let options = self
                                .render
                                .container
                                .get_resource::<CanvasOptions>()
                                .expect("no canvas options attached");
                            self.attach_canvas(futures::executor::block_on(Canvas::new(
                                &window.0,
                                options.clone(),
                            )));
                        }
                        self.compute.activate();
                        self.render.activate();
                    }
                }
                Event::MainEventsCleared => {
                    if self.compute.active() {
                        self.compute.exec(WorkloadId::Main);
                        self.render
                            .container
                            .get_resource::<CanvasWindow>()
                            .expect("no canvas window attached")
                            .0
                            .request_redraw();
                    }
                    if self.compute.should_exit() {
                        control_flow.set_exit();
                    }
                }
                Event::RedrawRequested(_window_id) => {
                    if self.render.active() {
                        self.extract();
                        self.render.exec(WorkloadId::Main);
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
}
