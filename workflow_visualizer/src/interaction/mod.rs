pub(crate) use attachment::InteractionAttachment;
pub(crate) use interaction::{MouseAdapter, resolve};
pub use interaction::{
    ActiveInteraction, Interactable, Interaction, InteractionDevice, InteractionEvent,
    InteractionLocation, InteractionLocations, InteractionPhase, InteractionPhases,
    InteractionTracker, PrimaryInteraction, PrimaryMouseButton, Toggled, Triggered,
};

mod attachment;
mod interaction;
