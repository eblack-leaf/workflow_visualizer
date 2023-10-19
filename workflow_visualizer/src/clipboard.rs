use crate::{Attach, Visualizer};
use bevy_ecs::system::Resource;
#[derive(Resource)]
pub struct Clipboard {
    #[cfg(not(target_family = "wasm"))]
    pub handle: Option<arboard::Clipboard>,
    #[cfg(target_family = "wasm")]
    pub handle: Option<()>,
}
impl Clipboard {
    #[cfg(not(target_family = "wasm"))]
    pub(crate) fn new() -> Self {
        let handle = arboard::Clipboard::new();
        Self {
            handle: if handle.is_ok() {
                Some(handle.expect("clipboard"))
            } else {
                None
            },
        }
    }
    #[cfg(target_family = "wasm")]
    pub(crate) fn new() -> Self {
        let handle = web_sys::window().expect("window").navigator().clipboard();
        Self {
            handle: if handle.is_some() { Some(()) } else { None },
        }
    }
    pub fn write(&mut self, data: String) {
        #[cfg(target_family = "wasm")]
        if let Some(h) = self.handle.as_ref() {
            h.write_text(data.as_str());
        }
        #[cfg(not(target_family = "wasm"))]
        if let Some(h) = self.handle.as_mut() {
            h.set_text(data).expect("clipboard writing");
        }
    }
}
pub(crate) struct ClipboardAttachment;
impl Attach for ClipboardAttachment {
    fn attach(visualizer: &mut Visualizer) {
        visualizer.job.container.insert_resource(Clipboard::new());
    }
}
