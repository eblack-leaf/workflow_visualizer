pub use attachment::FocusAttachment;
pub use component::{Focus, FocusedEntity};
pub(crate) use system::set_focused;

mod attachment;
mod component;
mod system;
