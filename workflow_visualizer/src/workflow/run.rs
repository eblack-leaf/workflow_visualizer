use std::rc::Rc;

use tracing::{info, trace};
use winit::event::{Event, StartCause, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoopWindowTarget};
use winit::window::Window;

use crate::workflow::native::initialize_native_window;
use crate::{Area, DeviceContext, Sender, Visualizer, Workflow};

pub(crate) fn internal_loop<T: Workflow + 'static>(
    mut visualizer: &mut Visualizer,
    mut window: &mut Option<Rc<Window>>,
    initialized: &mut bool,
    event: Event<<T as Workflow>::Response>,
    event_loop_window_target: &EventLoopWindowTarget<<T as Workflow>::Response>,
    control_flow: &mut ControlFlow,
    desktop_dimensions: Option<Area<DeviceContext>>,
) {
    if visualizer.can_idle() {
        control_flow.set_wait();
    } else {
        control_flow.set_poll();
    }
    match event {
        Event::NewEvents(cause) => match cause {
            StartCause::Init => {
                #[cfg(not(target_os = "android"))]
                {
                    #[cfg(not(target_family = "wasm"))]
                    initialize_native_window(event_loop_window_target, window, desktop_dimensions);
                    visualizer.initialize(window.as_ref().unwrap());
                    *initialized = true;
                }
            }
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
                let scale_factor = window.as_ref().unwrap().scale_factor();
                visualizer.trigger_resize(size, scale_factor);
            }
            WindowEvent::ScaleFactorChanged {
                new_inner_size,
                scale_factor,
            } => {
                info!("resizing: {:?}", *new_inner_size);
                visualizer.trigger_resize(*new_inner_size, scale_factor);
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
            WindowEvent::ReceivedCharacter(ch) => {
                trace!("char: {:?}", ch);
            }
            _ => {}
        },
        Event::UserEvent(event) => {
            let message = format!("visualizer loop received: {:?}", event);
            info!(message);
            #[cfg(target_family = "wasm")]
            gloo_console::info!(wasm_bindgen::JsValue::from_str(message.as_str()));
            if T::is_exit_response(&event) {
                control_flow.set_exit();
            }
            T::handle_response(visualizer, event);
        }
        Event::MainEventsCleared => {
            visualizer.exec();
            if visualizer.job.should_exit() {
                visualizer
                    .job
                    .container
                    .get_non_send_resource_mut::<Sender<T>>()
                    .expect("sender")
                    .send(T::exit_action());
            }
        }
        Event::RedrawRequested(_) => {
            visualizer.render();
        }
        Event::RedrawEventsCleared => {
            if visualizer.job.resumed() && *initialized {
                window.as_ref().unwrap().request_redraw();
            }
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
        Event::LoopDestroyed => {
            visualizer.teardown();
        }
        _ => {}
    }
}
