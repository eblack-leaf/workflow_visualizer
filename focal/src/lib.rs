use winit::event_loop::EventLoop;
use winit::window::Window;

use crate::canvas::Canvas;
use crate::job::Job;

mod canvas;
mod job;

#[cfg(test)]
#[test]
fn entry() {
    let job = Job::new();
    let mut visualizer = Visualizer::new();
    visualizer.launch(job);
}

pub struct VisualizerOptions {}

impl VisualizerOptions {
    pub fn native() -> Self {
        Self {}
    }
    pub fn web() -> Self {
        Self {}
    }
}

pub struct Visualizer {
    pub options: VisualizerOptions,
    pub event_loop: Option<EventLoop<()>>,
    pub window: Option<Window>,
    pub canvas: Option<Canvas>,
}

impl Visualizer {
    pub fn new() -> Self {
        Self {
            options: {
                let mut options = VisualizerOptions::native();
                #[cfg(target_arch = "wasm32")]
                {
                    options = VisualizerOptions::web();
                }
                options
            },
            event_loop: None,
            window: None,
            canvas: None,
        }
    }
    pub fn attach_window(&mut self, window: Window) {}
    pub fn detach_window(&mut self) -> Option<Window> {
        self.window.take()
    }
    pub fn attach_canvas(&mut self, canvas: Canvas) {}
    pub fn detach_canvas(&mut self) -> Option<Canvas> {
        self.canvas.take()
    }
    pub fn attach_event_loop(&mut self, event_loop: EventLoop<()>) {}
    pub async fn create_canvas(&self, window: &Window) -> Canvas {
        Canvas::new().await
    }
    pub fn launch(&mut self, job: Job) {
        self.event_loop
            .expect("no event loop to launch from")
            .run(|event, _event_loop_window_target, control_flow| {});
    }
}
