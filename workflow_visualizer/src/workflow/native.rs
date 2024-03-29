#[cfg(not(target_family = "wasm"))]
use std::rc::Rc;
#[cfg(not(target_family = "wasm"))]
use std::sync::{Arc, Mutex};
#[cfg(not(target_family = "wasm"))]
use winit::dpi::PhysicalSize;
#[cfg(not(target_family = "wasm"))]
use winit::event_loop::{EventLoopBuilder, EventLoopWindowTarget};
#[cfg(target_os = "android")]
use winit::platform::android::activity::AndroidApp;
#[cfg(not(target_family = "wasm"))]
use winit::window::{Window, WindowBuilder};

#[cfg(not(target_family = "wasm"))]
use crate::workflow::bridge::NativeSender;
#[cfg(not(target_family = "wasm"))]
use crate::workflow::bridge::{Receiver, Responder};
#[cfg(not(target_family = "wasm"))]
use crate::workflow::run::internal_loop;
#[cfg(not(target_family = "wasm"))]
use crate::workflow::runner::EngenHandle;
#[cfg(not(target_family = "wasm"))]
use crate::{Area, DeviceContext, Runner, Sender, Visualizer, Workflow};

#[cfg(not(target_family = "wasm"))]
pub(crate) fn internal_native_run<T: Workflow + Send + 'static>(
    runner: Runner,
    mut visualizer: Visualizer,
) {
    let tokio_runtime = tokio::runtime::Runtime::new().expect("tokio runtime");
    tokio_runtime.block_on(async {
        let builder = &mut EventLoopBuilder::<T::Response>::with_user_event();
        #[cfg(target_os = "android")]
        {
            use winit::platform::android::EventLoopBuilderExtAndroid;
            let android_app = runner.android_app.take().unwrap();
            builder.with_android_app(android_app.clone());
            visualizer
                .job
                .container
                .insert_resource(AndroidInterface(android_app.clone()));
        }
        let event_loop = builder.build().expect("event-loop");
        let (sender, receiver): (
            tokio::sync::mpsc::UnboundedSender<T::Action>,
            tokio::sync::mpsc::UnboundedReceiver<T::Action>,
        ) = tokio::sync::mpsc::unbounded_channel();
        let proxy = event_loop.create_proxy();
        tokio::task::spawn(async move {
            let engen = EngenHandle(Arc::new(Mutex::new(T::default())));
            let mut receiver = Receiver { receiver };
            let responder = Responder(proxy);
            loop {
                while let Some(action) = receiver.receive().await {
                    let response = T::handle_action(engen.0.clone(), action).await;
                    responder.respond(response);
                }
            }
        });
        visualizer
            .job
            .container
            .insert_non_send_resource(Sender::new(NativeSender::<T>::new(sender)));
        let mut window: Option<Rc<Window>> = None;
        let mut initialized = false;
        let desktop_dimensions = runner._desktop_dimensions;
        let _ = event_loop.run(move |event, event_loop_window_target| {
            internal_loop::<T>(
                &mut visualizer,
                &mut window,
                &mut initialized,
                event,
                event_loop_window_target,
                desktop_dimensions,
            );
        });
    });
}
#[cfg(not(target_family = "wasm"))]
pub(crate) fn initialize_native_window<T>(
    w_target: &EventLoopWindowTarget<T>,
    window: &mut Option<Rc<Window>>,
    desktop_dimensions: Option<Area<DeviceContext>>,
) {
    #[allow(unused_mut)]
    let mut builder = WindowBuilder::new().with_resizable(false);
    #[cfg(all(not(target_os = "android"), not(target_family = "wasm")))]
    {
        let desktop_dimensions = match desktop_dimensions {
            None => Area::new(600.0, 800.0),
            Some(dim) => dim,
        };
        builder = builder.with_inner_size(PhysicalSize::new(
            desktop_dimensions.width,
            desktop_dimensions.height,
        ));
    }
    window.replace(Rc::new(builder.build(w_target).expect("window")));
}
/// Interface to the Android system
#[cfg(target_os = "android")]
#[derive(Resource)]
pub struct AndroidInterface(pub AndroidApp);
