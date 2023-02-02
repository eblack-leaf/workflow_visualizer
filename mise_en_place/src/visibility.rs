use std::collections::{HashMap, HashSet};

use bevy_ecs::prelude::{
    Commands, Component, Entity, EventReader, IntoSystemDescriptor, Query, Res, ResMut, Resource,
    SystemLabel,
};

use crate::{Attach, BackendStages, FrontEndStages, Job, Stove};
use crate::coord::{Area, Position, PositionAdjust, ScaledArea, ScaledPosition, Section};
use crate::extract::Extract;
use crate::gfx::{GfxSurface, GfxSurfaceConfiguration};
use crate::viewport::Viewport;
use crate::window::{Resize, ScaleFactor};

#[derive(Component)]
pub(crate) struct Visibility {
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
pub(crate) struct VisibleSection {
    pub(crate) section: Section,
}

impl VisibleSection {
    pub(crate) fn new(section: Section) -> Self {
        Self { section }
    }
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub(crate) struct SpacialHash {
    pub(crate) x: u32,
    pub(crate) y: u32,
}

#[derive(Resource)]
pub(crate) struct SpacialHasher {
    pub(crate) alignment: f32,
    pub(crate) cached_hash_range: (),
    pub(crate) entities: HashMap<SpacialHash, HashSet<Entity>>,
}

impl SpacialHasher {
    pub(crate) fn new() -> Self {
        todo!()
    }
}

pub(crate) fn calc_visible_section(
    visible_bounds: Res<VisibleBounds>,
    mut entities: Query<(Entity, &mut Visibility, &mut VisibleSection)>,
    mut spacial_hash: ResMut<SpacialHasher>,
) {
    // for cached hash range
    // check visibility and update
    // for new range
    // check visibility and update
}

#[derive(Resource)]
pub struct VisibleBounds {
    pub(crate) section: Section,
    dirty: bool,
}

impl VisibleBounds {
    pub(crate) fn new(section: Section) -> Self {
        Self {
            section,
            dirty: false,
        }
    }
    pub fn position_adjust(&mut self, adjust: PositionAdjust) {
        self.section.position.adjust(adjust);
        self.dirty = true;
    }
    pub fn adjust_area(&mut self, area: Area) {
        self.section.area = area;
    }
}

#[derive(Resource)]
pub struct VisibleBoundsPositionAdjust {
    pub adjust: Option<PositionAdjust>,
}

impl VisibleBoundsPositionAdjust {
    pub(crate) fn new() -> Self {
        Self { adjust: None }
    }
}

pub fn adjust_position(
    mut visible_bounds: ResMut<VisibleBounds>,
    mut visible_bounds_position_adjust: ResMut<VisibleBoundsPositionAdjust>,
) {
    if let Some(adjust) = visible_bounds_position_adjust.adjust.take() {
        visible_bounds.position_adjust(adjust);
    }
}

#[derive(Resource)]
pub(crate) struct ViewportOffsetUpdate {
    pub(crate) update: Option<ScaledPosition>,
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
        let visible_bounds = frontend
            .container
            .get_resource::<VisibleBounds>()
            .expect("no visible bounds");
        let scale_factor = frontend
            .container
            .get_resource::<ScaleFactor>()
            .expect("no scale factor")
            .factor;
        if visible_bounds.dirty {
            backend
                .container
                .get_resource_mut::<ViewportOffsetUpdate>()
                .expect("no viewport offset update")
                .update
                .replace(visible_bounds.section.position.to_scaled(scale_factor));
        }
    }
}

pub(crate) fn resize(
    mut resize_events: EventReader<Resize>,
    mut visible_bounds: ResMut<VisibleBounds>,
    scale_factor: Res<ScaleFactor>,
) {
    for event in resize_events.iter() {
        visible_bounds.adjust_area(event.size.to_area(scale_factor.factor));
    }
}

impl Attach for Visibility {
    fn attach(stove: &mut Stove) {
        stove
            .backend
            .container
            .insert_resource(ViewportOffsetUpdate::new());
        stove.add_extraction::<VisibleBounds>();
        let gfx_surface_configuration = stove
            .backend
            .container
            .get_resource::<GfxSurfaceConfiguration>()
            .expect("no gfx surface config");
        let scale_factor = stove
            .frontend
            .container
            .get_resource::<ScaleFactor>()
            .expect("no scale factor")
            .factor;
        let area: ScaledArea = (
            gfx_surface_configuration.configuration.width,
            gfx_surface_configuration.configuration.height,
        )
            .into();
        stove.frontend.container.insert_resource(VisibleBounds::new(
            ((0u32, 0u32), area.to_area(scale_factor)).into(),
        ));
        stove
            .frontend
            .container
            .insert_resource(VisibleBoundsPositionAdjust::new());
        stove
            .backend
            .main
            .add_system_to_stage(BackendStages::Resize, viewport_read_offset);
        stove
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::Resize, resize);
        stove.frontend.main.add_system_to_stage(
            FrontEndStages::ResolveVisibility,
            adjust_position.label(VisibilitySystems::AdjustPosition),
        );
        stove.frontend.main.add_system_to_stage(
            FrontEndStages::ResolveVisibility,
            calc_visible_section.after(VisibilitySystems::AdjustPosition),
        );
    }
}

#[derive(SystemLabel)]
pub enum VisibilitySystems {
    AdjustPosition,
}
