pub use attachment::ClickableAttachment;
pub(crate) use components::TrackedClick;
pub use components::{ClickListener, ClickState, Clickable, DisableClick};
pub(crate) use system::register_click;

mod attachment;
mod components;
mod system;
