use bevy_ecs::change_detection::ResMut;
use bevy_ecs::prelude::{Res, Resource};

use crate::gfx::{Extract, GfxSurface};
use crate::visibility::spacial_hasher::SpacialHasher;
use crate::window::ScaleFactor;
use crate::{Area, DeviceView, Job, Position, PositionAdjust, Section, UIView, Viewport};

#[derive(Resource)]
pub struct VisibleBounds {
    pub(crate) section: Section<UIView>,
    dirty: bool,
}

impl VisibleBounds {
    pub(crate) fn new(section: Section<UIView>) -> Self {
        Self {
            section,
            dirty: false,
        }
    }
    pub fn position_adjust(&mut self, adjust: PositionAdjust<UIView>) {
        self.section.position.adjust(adjust);
        self.dirty = true;
    }
    pub fn adjust_area(&mut self, area: Area<UIView>) {
        self.section.area = area;
    }
}

#[derive(Resource)]
pub struct VisibleBoundsPositionAdjust {
    pub adjust: Option<PositionAdjust<UIView>>,
}

impl VisibleBoundsPositionAdjust {
    pub(crate) fn new() -> Self {
        Self { adjust: None }
    }
}

pub(crate) fn adjust_position(
    mut visible_bounds: ResMut<VisibleBounds>,
    mut visible_bounds_position_adjust: ResMut<VisibleBoundsPositionAdjust>,
    mut spacial_hasher: ResMut<SpacialHasher>,
) {
    if let Some(adjust) = visible_bounds_position_adjust.adjust.take() {
        visible_bounds.position_adjust(adjust);
        spacial_hasher.visible_bounds_changed = true;
    }
}

#[derive(Resource)]
pub(crate) struct ViewportOffsetUpdate {
    pub(crate) update: Option<Position<DeviceView>>,
}

impl ViewportOffsetUpdate {
    pub(crate) fn new() -> Self {
        Self { update: None }
    }
}

pub(crate) fn viewport_read_offset(
    mut viewport_offset_update: ResMut<ViewportOffsetUpdate>,
    mut viewport: ResMut<Viewport>,
    gfx_surface: Res<GfxSurface>,
) {
    if let Some(update) = viewport_offset_update.update.take() {
        viewport.update_offset(&gfx_surface.queue, update);
    }
}

impl Extract for VisibleBounds {
    fn extract(frontend: &mut Job, backend: &mut Job) {
        let scale_factor = frontend
            .container
            .get_resource::<ScaleFactor>()
            .expect("no scale factor")
            .factor;
        let mut visible_bounds = frontend
            .container
            .get_resource_mut::<VisibleBounds>()
            .expect("no visible bounds");
        if visible_bounds.dirty {
            backend
                .container
                .get_resource_mut::<ViewportOffsetUpdate>()
                .expect("no viewport offset update")
                .update
                .replace(visible_bounds.section.position.to_device(scale_factor));
            visible_bounds.dirty = false;
        }
    }
}
