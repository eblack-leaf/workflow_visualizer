use winit::event::{Event, StartCause, WindowEvent};

use crate::{
    Engen, gfx, IconAttachment, PositionAdjustAnimator, TextAttachment, ViewportAttachment,
};
use crate::clickable::ClickableAttachment;
use crate::coord::CoordAttachment;
use crate::engen::TaskLabel;
use crate::focus::FocusAttachment;
use crate::text_input::TextInputAttachment;
use crate::time::TimerAttachment;
use crate::visibility::VisibilityAttachment;
use crate::window::WindowAttachment;

pub(crate) fn ignite(mut engen: Engen) {
    let event_loop = engen.event_loop.take().expect("no event loop");
    let mut displayed = false;
    event_loop.run(
        move |event, event_loop_window_target, control_flow| match event {
            Event::NewEvents(start_cause) => match start_cause {
                StartCause::Init => {
                    engen.init_native_gfx(event_loop_window_target);
                    engen.invoke_attach::<TimerAttachment>();
                    engen.invoke_attach::<CoordAttachment>();
                    engen.invoke_attach::<WindowAttachment>();
                    engen.invoke_attach::<ViewportAttachment>();
                    engen.invoke_attach::<VisibilityAttachment>();
                    engen.invoke_attach::<ClickableAttachment>();
                    engen.invoke_attach::<FocusAttachment>();
                    engen.invoke_attach::<IconAttachment>();
                    engen.invoke_attach::<TextAttachment>();
                    engen.invoke_attach::<PositionAdjustAnimator>();
                    engen.invoke_attach::<TextInputAttachment>();
                    engen.attach_from_queue();
                    engen.frontend.exec(TaskLabel::Startup);
                    engen.backend.exec(TaskLabel::Startup);
                }
                _ => {}
            },
            Event::WindowEvent {
                window_id: _window_id,
                event: w_event,
            } => match w_event {
                WindowEvent::Resized(size) => {
                    let scale_factor = engen.window.as_ref().expect("no window").scale_factor();
                    engen.resize_callback(size, scale_factor);
                }
                WindowEvent::ScaleFactorChanged {
                    new_inner_size,
                    scale_factor,
                } => {
                    engen.resize_callback(*new_inner_size, scale_factor);
                }
                WindowEvent::CloseRequested => {
                    control_flow.set_exit();
                }
                WindowEvent::Touch(touch) => {
                    engen.register_touch(touch);
                }
                WindowEvent::CursorMoved {
                    device_id: _device_id,
                    position,
                    ..
                } => {
                    engen.set_mouse_location(position);
                }
                WindowEvent::MouseInput {
                    device_id: _device_id,
                    state,
                    button,
                    ..
                } => {
                    engen.register_mouse_click(state, button);
                }
                _ => {}
            },
            Event::Suspended => {
                if engen.frontend.active() {
                    #[cfg(target_os = "android")]
                    {
                        let _ = engen.backend.container.remove_resource::<GfxSurface>();
                    }
                    engen.frontend.suspend();
                    engen.backend.suspend();
                }
            }
            Event::Resumed => {
                if engen.frontend.suspended() {
                    #[cfg(target_os = "android")]
                    {
                        let window = engen.window.as_ref().expect("no window");
                        let gfx = futures::executor::block_on(GfxSurface::new(
                            &window,
                            GfxOptions::native(),
                        ));
                        engen.backend.container.insert_resource(gfx.0);
                        engen.backend.container.insert_resource(gfx.1);
                    }
                    engen.frontend.activate();
                    engen.backend.activate();
                }
            }
            Event::MainEventsCleared => {
                if engen.frontend.active() {
                    engen.frontend.exec(TaskLabel::Main);
                    if !displayed {
                        // let dag = engen.frontend.main.graph().dependency().cached_topsort();
                        // for node in dag.iter() {
                        //     println!("node: {:?}", node);
                        // }
                        displayed = true;
                    }
                }
                if engen.frontend.should_exit() {
                    control_flow.set_exit();
                }
            }
            Event::RedrawRequested(_) => {
                if engen.backend.active() {
                    gfx::extract(&mut engen);
                    engen.backend.exec(TaskLabel::Main);
                    gfx::render(&mut engen);
                }
            }
            Event::RedrawEventsCleared => {
                if engen.backend.active() {
                    engen.window.as_ref().expect("no window").request_redraw();
                }
                if engen.frontend.can_idle() && engen.backend.can_idle() {
                    control_flow.set_wait();
                }
            }
            Event::LoopDestroyed => {
                engen.frontend.exec(TaskLabel::Teardown);
                engen.backend.exec(TaskLabel::Teardown);
            }
            _ => {}
        },
    );
}
