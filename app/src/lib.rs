pub mod canvas;
pub mod job;
pub mod viewport;

use crate::canvas::Canvas;
use crate::job::Job;
use winit::event_loop::EventLoop;
use winit::window::Window;
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
    pub async fn run<T>(&mut self, event_loop: EventLoop<T>) {}
}
