use bevy_ecs::prelude::Component;

pub(crate) use attachment::VisibilityAttachment;
pub use collision::{Collision, CollisionBegin, CollisionEnd};
pub(crate) use visible_bounds::ViewportOffsetUpdate;
pub use visible_bounds::{VisibleBounds, VisibleBoundsPositionAdjust};

use crate::coord::{Section, UIView};

mod attachment;
mod collision;
mod spacial_hasher;
mod system;
mod visible_bounds;

#[derive(Component, Copy, Clone)]
pub struct Visibility {
    visible: bool,
}

impl Visibility {
    pub(crate) fn new() -> Self {
        Self { visible: false }
    }
    pub fn visible(&self) -> bool {
        self.visible
    }
}

#[derive(Component, Copy, Clone)]
pub struct VisibleSection {
    pub(crate) section: Section<UIView>,
}

impl VisibleSection {
    pub(crate) fn new(section: Section<UIView>) -> Self {
        Self { section }
    }
    pub fn section(&self) -> Section<UIView> {
        self.section
    }
}
