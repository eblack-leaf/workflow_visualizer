pub mod canvas;
pub mod color;
pub mod coord;
pub mod depth_texture;
mod gpu_bindings;
pub mod job;
mod renderer;
pub mod theme;
mod uniform;
pub mod viewport;
pub mod window;

use crate::canvas::Canvas;
use crate::job::{ExecutionState, Job};
use crate::window::Resize;
use bevy_ecs::system::Resource;
use winit::event::{Event, StartCause, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::Window;

#[derive(Clone)]
pub struct Signal<T: Send + Sync + 'static> {
    pub signal: Option<T>,
}
impl<T: Send + Sync + 'static> Signal<T> {
    pub fn new(signal: Option<T>) -> Self {
        Self { signal }
    }
    pub fn receive(&mut self) -> Option<T> {
        return self.signal.take();
    }
    pub fn emit(&mut self, signal: T) {
        self.signal = Some(signal);
    }
}
pub struct WakeMessage {}
pub struct App {
    pub compute: Job,
    pub render: Job,
}
impl App {
    pub fn new() -> Self {
        Self {
            compute: Job::new(),
            render: Job::new(),
        }
    }
    pub fn attach_window(&mut self, window: Window) {
        self.render.container.insert_non_send_resource(window);
    }
    pub fn detach_window(&mut self) -> Window {
        return self
            .render
            .container
            .remove_non_send_resource::<Window>()
            .expect("no window to detach");
    }
    pub fn attach_canvas(&mut self, canvas: Canvas) {
        self.render.container.insert_resource(canvas.0);
        self.render.container.insert_resource(canvas.1);
        self.render.container.insert_resource(canvas.2);
        self.render.container.insert_resource(canvas.3);
    }
    pub fn detach_canvas(&mut self) -> Canvas {
        return (
            self.render
                .container
                .remove_resource::<wgpu::Surface>()
                .expect("error detaching"),
            self.render
                .container
                .remove_resource::<wgpu::Device>()
                .expect("error detaching"),
            self.render
                .container
                .remove_resource::<wgpu::Queue>()
                .expect("error detaching"),
            self.render
                .container
                .remove_resource::<wgpu::SurfaceConfiguration>()
                .expect("error detaching"),
        );
    }
    pub fn request_redraw(&self) {
        self.render
            .container
            .get_non_send_resource::<Window>()
            .expect("no window in container")
            .request_redraw();
    }
    pub fn get_canvas_options(&self) -> canvas::Options {
        return self
            .render
            .container
            .get_resource::<canvas::Options>()
            .expect("no canvas options configured")
            .clone();
    }
    pub fn set_canvas_options(&mut self, options: canvas::Options) {
        self.render.container.insert_resource(options);
    }
    pub fn can_idle(&self) -> bool {
        return self.compute.can_idle() && self.render.can_idle();
    }
    pub fn extract_render_packets(&mut self) {}
    pub fn render_post_processing(&mut self) {}
}
pub async fn run<T>(mut app: App, event_loop: EventLoop<T>) {
    event_loop.run(
        move |event, _event_loop_window_target, control_flow| {
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
                            let options = app.get_canvas_options();
                            app.attach_canvas(futures::executor::block_on(canvas::canvas(
                                &window, options,
                            )));
                            app.attach_window(window);
                        }
                        app.compute.startup();
                        app.render.startup();
                    }
                },
                Event::WindowEvent { window_id, event } => match event {
                    WindowEvent::Resized(physical_size) => {
                        app.render.emit(Resize::new((
                            physical_size.width as f32,
                            physical_size.height as f32,
                        )));
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
                        app.render.emit(Resize::new((
                            new_inner_size.width as f32,
                            new_inner_size.height as f32,
                        )));
                    }
                    WindowEvent::ThemeChanged(_) => {}
                    WindowEvent::Occluded(_) => {}
                },
                Event::DeviceEvent { .. } => {}
                Event::UserEvent(_) => {}
                Event::Suspended => {
                    #[cfg(target_os = "android")]
                    {
                        if app.render.active() {
                            let _ = app.detach_canvas();
                        }
                    }
                    app.compute.suspend();
                    app.render.suspend();
                }
                Event::Resumed => {
                    #[cfg(target_os = "android")]
                    {
                        if app.render.suspended() {
                            let window = app.detach_window();
                            let options = app.get_canvas_options();
                            app.attach_canvas(futures::executor::block_on(canvas::canvas(
                                &window, options,
                            )));
                            app.attach_window(window);
                        }
                    }
                    app.compute.activate();
                    app.render.activate();
                }
                Event::MainEventsCleared => {
                    if app.compute.active() {
                        app.compute.exec();
                    }
                    if app.compute.should_exit() {
                        control_flow.set_exit();
                    }
                    app.request_redraw();
                }
                Event::RedrawRequested(_window_id) => {
                    if app.render.active() {
                        app.extract_render_packets();
                        app.render.exec();
                    }
                    if app.render.should_exit() {
                        control_flow.set_exit();
                    }
                }
                Event::RedrawEventsCleared => {
                    if app.render.active() {
                        app.render_post_processing();
                    }
                    if app.can_idle() {
                        control_flow.set_wait();
                    }
                }
                Event::LoopDestroyed => {
                    app.compute.teardown();
                    app.render.teardown();
                }
            }
        }
    );
}
