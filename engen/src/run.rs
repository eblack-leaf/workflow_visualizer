use crate::canvas::Canvas;
use crate::text::TextRenderer;
use crate::{render, Launcher};
use winit::event::{Event, StartCause, WindowEvent};
use winit::window::Window;

pub(crate) fn run(mut launcher: Launcher) {
    let event_loop = launcher.event_loop.take().expect("no event loop provided");
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
                        launcher.attach_canvas(futures::executor::block_on(Canvas::new(
                            &window,
                            launcher.options.clone(),
                        )));
                        launcher.attach_window(window);
                    }
                    launcher.compute.job.startup();
                    launcher.attach_renderer::<TextRenderer>();
                }
            },
            Event::WindowEvent {
                window_id: _window_id,
                event,
            } => match event {
                WindowEvent::Resized(physical_size) => {
                    launcher
                        .canvas
                        .as_mut()
                        .unwrap()
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
                    launcher
                        .canvas
                        .as_mut()
                        .unwrap()
                        .adjust(new_inner_size.width, new_inner_size.height);
                }
                WindowEvent::ThemeChanged(_) => {}
                WindowEvent::Occluded(_) => {}
            },
            Event::DeviceEvent { .. } => {}
            Event::UserEvent(_) => {}
            Event::Suspended => {
                if launcher.compute.job.active() {
                    #[cfg(target_os = "android")]
                    {
                        let _ = launcher.detach_canvas();
                    }
                    launcher.compute.job.suspend();
                }
            }
            Event::Resumed => {
                if launcher.compute.job.suspended() {
                    #[cfg(target_os = "android")]
                    {
                        let window = launcher.detach_window();
                        launcher.attach_canvas(futures::executor::block_on(Canvas::new(
                            &window,
                            launcher.options.clone(),
                        )));
                        launcher.attach_window(window);
                    }
                    launcher.compute.job.activate();
                }
            }
            Event::MainEventsCleared => {
                if launcher.compute.job.active() {
                    launcher.compute.job.exec();
                    launcher.window.as_ref().unwrap().0.request_redraw();
                }
                if launcher.compute.job.should_exit() {
                    control_flow.set_exit();
                }
            }
            Event::RedrawRequested(_window_id) => {
                if launcher.compute.job.active() {
                    render::extract(&mut launcher.renderers, &mut launcher.compute);
                    render::prepare(&mut launcher.renderers, launcher.canvas.as_ref().unwrap());
                    render::render(
                        &mut launcher.renderers,
                        launcher.canvas.as_ref().unwrap(),
                        &launcher.theme,
                    );
                }
            }
            Event::RedrawEventsCleared => {
                if launcher.compute.job.can_idle() {
                    control_flow.set_wait();
                }
            }
            Event::LoopDestroyed => {
                launcher.compute.job.teardown();
            }
        }
    });
}
