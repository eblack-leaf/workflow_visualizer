use std::sync::{Arc, Mutex};
#[cfg(not(target_family = "wasm"))]
use crate::workflow::bridge::NativeSender;
use crate::workflow::bridge::{OutputWrapper, Receiver, Responder, Sender};
#[cfg(not(target_family = "wasm"))]
use crate::workflow::native::internal_native_run;
#[cfg(target_family = "wasm")]
use crate::workflow::web::internal_web_run;
#[cfg(target_os = "android")]
use winit::platform::android::activity::AndroidApp;
use crate::{Area, DeviceContext, Visualizer, Workflow};

pub(crate) struct EngenHandle<T: Workflow + Default>(pub(crate) Arc<Mutex<T>>);
pub struct Runner {
    pub(crate) desktop_dimensions: Option<Area<DeviceContext>>,
    #[cfg(not(target_os = "android"))]
    pub(crate) android_app: Option<()>,
    #[cfg(target_os = "android")]
    pub(crate) android_app: Option<AndroidApp>,
}

impl Default for Runner {
    fn default() -> Self {
        Self::new()
    }
}

impl Runner {
    pub fn new() -> Self {
        Self {
            desktop_dimensions: None,
            android_app: None,
        }
    }
    #[cfg(target_os = "android")]
    pub fn with_android_app(mut self, android_app: AndroidApp) -> Self {
        self.android_app.replace(android_app);
        self
    }
    pub fn with_desktop_dimensions<A: Into<Area<DeviceContext>>>(mut self, dim: A) -> Self {
        self.desktop_dimensions.replace(dim.into());
        self
    }
    #[cfg(not(target_family = "wasm"))]
    pub fn native_run<T: Workflow + Send + 'static + Default>(
        mut self,
        mut visualizer: Visualizer,
    ) {
        internal_native_run::<T>(self, visualizer);
    }

    #[cfg(target_family = "wasm")]
    pub fn web_run<T: Workflow + 'static + Default>(
        mut self,
        visualizer: Visualizer,
        worker_path: String,
    ) {
        #[cfg(target_family = "wasm")]
        wasm_bindgen_futures::spawn_local(internal_web_run::<T>(self, visualizer, worker_path));
    }
}
