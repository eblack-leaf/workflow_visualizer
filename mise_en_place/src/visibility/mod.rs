use std::collections::{HashMap, HashSet};

use bevy_ecs::prelude::{
    Added, Changed, Commands, Component, Entity, EventReader, IntoSystemDescriptor, Or, Query,
    RemovedComponents, Res, ResMut, Resource, SystemLabel, With, Without,
};
pub use collision::{Collision, CollisionBegin, CollisionEnd};
use spacial_hasher::SpacialHasher;
pub(crate) use visible_bounds::ViewportOffsetUpdate;
pub use visible_bounds::{VisibleBounds, VisibleBoundsPositionAdjust};
pub(crate) use plugin::VisibilityPlugin;
use crate::coord::{Area, DeviceView, Position, PositionAdjust, Section, UIView};
use crate::gfx::Extract;
use crate::gfx::{GfxSurface, GfxSurfaceConfiguration};
use crate::window::{Resize, ScaleFactor};
use crate::{Attach, BackendStages, Engen, FrontEndStages, Job, Viewport};

mod collision;
mod spacial_hasher;
mod visible_bounds;
mod plugin;
mod system;

#[derive(Component)]
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
