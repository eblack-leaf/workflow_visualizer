pub use attachment::{ClickSystems, ClickableAttachment};
pub(crate) use components::TrackedClick;
pub use components::{ClickListener, ClickState, Clickable, DisableClick};

mod attachment;
mod components;
mod system;
