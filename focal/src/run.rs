use winit::event::{Event, StartCause, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::Window;
use crate::{canvas, Gfx, GfxOptions, Job};
use crate::canvas::Canvas;
use crate::render::render;
use crate::viewport::Viewport;
pub(crate) async fn web_run(mut gfx: Gfx, job: Job) {
    #[cfg(target_arch = "wasm32")] {
        gfx.set_options(GfxOptions::web());
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
        gfx.attach_canvas(Canvas::new(&window, gfx.options.clone()).await);
        gfx.attach_window(window);
        gfx.attach_event_loop(event_loop);
        run(gfx, job);
    }
}
pub(crate) fn run(mut gfx: Gfx, mut job: Job) {
    let event_loop = gfx.event_loop.take().unwrap();
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
                        gfx.attach_canvas(futures::executor::block_on(
                            Canvas::new(&window, gfx.options.clone()),
                        ));
                        gfx.attach_window(window);
                    }
                    job.startup();
                    let canvas = gfx.canvas.as_ref().unwrap();
                    let configuration = &canvas.surface_configuration;
                    gfx.viewport = Option::from(Viewport::new(
                        &canvas.device,
                        (configuration.width, configuration.height).into(),
                    ));
                }
            },
            Event::WindowEvent { window_id, event } => match event {
                WindowEvent::Resized(physical_size) => {
                    canvas::adjust(&mut gfx, physical_size.width, physical_size.height);
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
                    canvas::adjust(&mut gfx, new_inner_size.width, new_inner_size.height);
                }
                WindowEvent::ThemeChanged(_) => {}
                WindowEvent::Occluded(_) => {}
            },
            Event::DeviceEvent { .. } => {}
            Event::UserEvent(_) => {}
            Event::Suspended => {
                if job.active() {
                    #[cfg(target_os = "android")]
                    {
                        let _ = gfx.detach_canvas();
                    }
                    job.suspend();
                }
            }
            Event::Resumed => {
                if job.suspended() {
                    #[cfg(target_os = "android")]
                    {
                        let window = gfx.detach_window();
                        gfx.attach_canvas(futures::executor::block_on(
                            Canvas::new(&window, gfx.options.clone()),
                        ));
                        gfx.attach_window(window);
                    }
                    job.activate();
                }
            }
            Event::MainEventsCleared => {
                if job.active() {
                    job.exec();
                    gfx.window.as_ref().unwrap().request_redraw();
                }
                if job.should_exit() {
                    control_flow.set_exit();
                }

            }
            Event::RedrawRequested(_window_id) => {
                render(&mut gfx);
            }
            Event::RedrawEventsCleared => {
                if job.can_idle() {
                    control_flow.set_wait();
                }
            }
            Event::LoopDestroyed => {
                job.teardown();
            }
        }
    });
}