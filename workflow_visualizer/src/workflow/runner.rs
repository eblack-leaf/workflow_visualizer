#[cfg(not(target_family = "wasm"))]
use crate::workflow::bridge::NativeSender;
use crate::workflow::bridge::{OutputWrapper, Receiver, Responder, Sender};
#[cfg(not(target_family = "wasm"))]
use crate::workflow::native::internal_native_run;
#[cfg(target_family = "wasm")]
use crate::workflow::web::internal_web_run;
use crate::{Area, Attach, Attachment, DeviceContext, Visualizer, Workflow};
use std::sync::{Arc, Mutex};
#[cfg(target_os = "android")]
use winit::platform::android::activity::AndroidApp;

pub(crate) struct EngenHandle<T: Workflow + Default>(pub(crate) Arc<Mutex<T>>);
/// Main struct to run the visualizer's event loop
pub struct Runner {
    attachment_queue: Vec<Attachment>,
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
            attachment_queue: vec![],
            desktop_dimensions: None,
            android_app: None,
        }
    }
    /// insert the AndroidApp for interfacing with the Android OS
    #[cfg(target_os = "android")]
    pub fn with_android_app(mut self, android_app: AndroidApp) -> Self {
        self.android_app.replace(android_app);
        self
    }
    /// set the fixed dimensions on the desktop platforms
    pub fn with_desktop_dimensions<A: Into<Area<DeviceContext>>>(mut self, dim: A) -> Self {
        self.desktop_dimensions.replace(dim.into());
        self
    }
    pub fn add_attachment<Attached: Attach>(&mut self) {
        self.attachment_queue.push(Attachment::using::<Attached>());
    }
    pub fn with_attachment<Attached: Attach>(mut self) -> Self {
        self.add_attachment::<Attached>();
        self
    }
    /// invoke a native run of the visualizer
    #[cfg(not(target_family = "wasm"))]
    pub fn native_run<T: Workflow + Send + 'static + Default>(
        mut self,
        mut visualizer: Visualizer,
    ) {
        visualizer.add_attachments(self.attachment_queue.drain(..).collect());
        internal_native_run::<T>(self, visualizer);
    }
    /// invoke a wasm run of the visualizer
    #[cfg(target_family = "wasm")]
    pub fn web_run<T: Workflow + 'static + Default>(
        mut self,
        visualizer: Visualizer,
        worker_path: String,
    ) {
        visualizer.add_attachments(self.attachment_queue.drain(..).collect());
        #[cfg(target_family = "wasm")]
        wasm_bindgen_futures::spawn_local(internal_web_run::<T>(self, visualizer, worker_path));
    }
}
