mod workflow;

use crate::workflow::{Token, TokenName, TokenOtp};
use winit::dpi::PhysicalSize;
use winit::event::{Event, StartCause, WindowEvent};
use winit::event_loop::{EventLoopBuilder, EventLoopProxy};
use winit::window::WindowBuilder;
use wkflw::{GfxOptions, Theme, Visualizer};
use workflow::{Engen, Receivable, Sendable};

fn main() {
    let tokio_runtime = tokio::runtime::Runtime::new().expect("tokio runtime");
    tokio_runtime.block_on(async {
        let event_loop = EventLoopBuilder::<Sendable>::with_user_event().build();
        let mut window = None;
        let mut visualizer = Visualizer::new(Theme::default(), GfxOptions::native());
        let (sender, mut receiver): (
            tokio::sync::mpsc::UnboundedSender<Receivable>,
            tokio::sync::mpsc::UnboundedReceiver<Receivable>,
        ) = tokio::sync::mpsc::unbounded_channel();
        let proxy = event_loop.create_proxy();
        tokio::task::spawn(async move {
            let engen = Engen::new();
            loop {
                while let Some(message) = receiver.recv().await {
                    match message {
                        Receivable::ExitRequest => {
                            proxy.send_event(Sendable::ExitConfirmed).expect("proxy");
                        }
                        Receivable::AddToken((name, token)) => {
                            proxy.send_event(Sendable::TokenAdded(name)).expect("proxy");
                        }
                        Receivable::GenerateOtp(name) => {
                            let otp = "".to_string();
                            proxy
                                .send_event(Sendable::TokenOtp((name, TokenOtp(otp))))
                                .expect("proxy");
                        }
                        Receivable::RemoveToken(name) => {
                            proxy
                                .send_event(Sendable::TokenRemoved(name))
                                .expect("proxy");
                        }
                    }
                }
            }
        });
        event_loop.run(move |event, event_loop_window_target, control_flow| {
            control_flow.set_wait();
            match event {
                Event::NewEvents(cause) => match cause {
                    StartCause::Init => {
                        window.replace(
                            WindowBuilder::new()
                                .with_resizable(false)
                                .with_inner_size(PhysicalSize::new(400, 600))
                                .build(&event_loop_window_target)
                                .expect("window"),
                        );
                        visualizer.initialize(window.as_ref().unwrap());
                    }
                    _ => {}
                },
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        if let Ok(_) = sender.send(Receivable::ExitRequest) {
                            println!("sending is ok");
                        } else {
                            println!("could not send");
                        }
                    }
                    WindowEvent::ReceivedCharacter(ch) => {
                        if ch == 'a' {
                            sender
                                .send(Receivable::AddToken((
                                    TokenName("rose".to_string()),
                                    Token("15983".to_string()),
                                )))
                                .expect("sender");
                        }
                    }
                    _ => {}
                },
                Event::UserEvent(event) => {
                    println!("ui loop received: {:?}", event);
                    match event {
                        Sendable::ExitConfirmed => {
                            control_flow.set_exit();
                        }
                        Sendable::TokenAdded(name) => {
                            println!("token added: {:?}", name);
                        }
                        Sendable::TokenRemoved(name) => {
                            println!("token removed: {:?}", name);
                        }
                        Sendable::TokenOtp((name, otp)) => {
                            println!("token otp: {:?}:{:?}", name, otp);
                        }
                    }
                }
                Event::MainEventsCleared => {
                    visualizer.compute();
                }
                Event::RedrawRequested(_) => {
                    visualizer.render();
                }
                Event::RedrawEventsCleared => {
                    if visualizer.job.active() {
                        window.as_ref().unwrap().request_redraw();
                    }
                    if visualizer.can_idle() {
                        control_flow.set_wait();
                    }
                }
                Event::Suspended => {
                    #[cfg(target_os = "android")]
                    visualizer.suspend();
                }
                Event::Resumed => {
                    #[cfg(target_os = "android")]
                    visualizer.resume(window.as_ref().unwrap());
                }
                Event::LoopDestroyed => {
                    visualizer.teardown();
                }
                _ => {}
            }
        });
    });
}
