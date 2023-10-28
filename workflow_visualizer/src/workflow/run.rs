use std::rc::Rc;

#[cfg(not(target_family = "wasm"))]
use crate::workflow::native::initialize_native_window;
use crate::{Area, DeviceContext, Sender, Visualizer, Workflow};
use tracing::info;
use winit::event::{Event, StartCause, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoopWindowTarget};
use winit::window::Window;

pub(crate) fn internal_loop<T: Workflow + 'static>(
    visualizer: &mut Visualizer,
    window: &mut Option<Rc<Window>>,
    initialized: &mut bool,
    event: Event<<T as Workflow>::Response>,
    #[allow(unused)] event_loop_window_target: &EventLoopWindowTarget<<T as Workflow>::Response>,
    #[allow(unused)] desktop_dimensions: Option<Area<DeviceContext>>,
) {
    if visualizer.can_idle() {
        event_loop_window_target.set_control_flow(ControlFlow::Wait);
    } else {
        event_loop_window_target.set_control_flow(ControlFlow::Poll);
    }
    match event {
        Event::NewEvents(cause) => match cause {
            StartCause::Init => {}
            _ => {}
        },
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested => {
                visualizer
                    .job
                    .container
                    .get_non_send_resource_mut::<Sender<T>>()
                    .expect("sender")
                    .send(T::exit_action());
            }
            WindowEvent::Resized(size) => {
                info!("resizing: {:?}", size);
                let scale_factor = window.as_ref().unwrap().scale_factor() as f32;
                visualizer.trigger_resize(size, scale_factor);
            }
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                visualizer.set_scale_factor(scale_factor as f32);
            }
            WindowEvent::Touch(touch) => {
                visualizer.register_touch(touch);
                info!("touch {:?}", touch);
            }
            WindowEvent::MouseInput { state, button, .. } => {
                visualizer.register_mouse_click(state, button);
            }
            WindowEvent::MouseWheel { .. } => {}
            WindowEvent::CursorMoved { position, .. } => {
                visualizer.set_mouse_location(position);
            }
            WindowEvent::CursorEntered { device_id: _ } => {}
            WindowEvent::CursorLeft { device_id: _ } => {
                visualizer.cancel_touches();
            }

            WindowEvent::ActivationTokenDone { .. } => {}
            WindowEvent::Moved(_) => {}
            WindowEvent::Destroyed => {}
            WindowEvent::DroppedFile(_) => {}
            WindowEvent::HoveredFile(_) => {}
            WindowEvent::HoveredFileCancelled => {}
            WindowEvent::Focused(_) => {}
            WindowEvent::KeyboardInput { .. } => {}
            WindowEvent::ModifiersChanged(_) => {}
            WindowEvent::Ime(_) => {}
            WindowEvent::TouchpadMagnify { .. } => {}
            WindowEvent::SmartMagnify { .. } => {}
            WindowEvent::TouchpadRotate { .. } => {}
            WindowEvent::TouchpadPressure { .. } => {}
            WindowEvent::AxisMotion { .. } => {}
            WindowEvent::ThemeChanged(_) => {}
            WindowEvent::Occluded(_) => {}
            WindowEvent::RedrawRequested => {
                visualizer.render();
            }
        },
        Event::UserEvent(event) => {
            if T::is_exit_response(&event) {
                event_loop_window_target.exit();
            }
            T::handle_response(visualizer, event);
        }
        Event::Suspended => {
            info!("suspending");
            #[cfg(target_os = "android")]
            {
                visualizer.suspend();
            }
        }
        Event::Resumed => {
            info!("resuming");
            #[cfg(not(target_os = "android"))]
            {
                #[cfg(not(target_family = "wasm"))]
                initialize_native_window(event_loop_window_target, window, desktop_dimensions);
                visualizer.initialize(window.as_ref().unwrap());
                *initialized = true;
            }
            #[cfg(target_os = "android")]
            {
                if !*initialized {
                    initialize_native_window(
                        &event_loop_window_target,
                        &mut window,
                        desktop_dimensions,
                    );
                    visualizer.initialize(window.as_ref().unwrap());
                    *initialized = true;
                } else {
                    visualizer.resume(window.as_ref().unwrap());
                }
            }
        }
        Event::DeviceEvent { .. } => {}
        Event::AboutToWait => {
            // does this get triggered many times and should be limited with bool like initialized?
            // if so reset exec_trigger here
            visualizer.exec();
            if visualizer.job.should_exit() {
                visualizer
                    .job
                    .container
                    .get_non_send_resource_mut::<Sender<T>>()
                    .expect("sender")
                    .send(T::exit_action());
            }
            if visualizer.job.resumed() && *initialized {
                window.as_ref().unwrap().request_redraw();
            }
        }
        Event::LoopExiting => {
            visualizer.teardown();
        }
        Event::MemoryWarning => {}
    }
}
