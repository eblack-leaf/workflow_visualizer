use std::collections::{HashMap, HashSet};

use bevy_ecs::prelude::{
    Changed, Commands, Component, Entity, EventReader, IntoSystemDescriptor, Or, Query, Res,
    ResMut, Resource, SystemLabel,
};

use crate::coord::{Area, Position, PositionAdjust, ScaledArea, ScaledPosition, Section};
use crate::extract::Extract;
use crate::gfx::{GfxSurface, GfxSurfaceConfiguration};
use crate::viewport::Viewport;
use crate::window::{Resize, ScaleFactor};
use crate::{Attach, BackendStages, FrontEndStages, Job, Stove};

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
impl SpacialHash {
    pub(crate) fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}
#[derive(Eq, Copy, Clone, PartialEq)]
pub(crate) struct SpacialHashRange {
    pub(crate) left: u32,
    pub(crate) top: u32,
    pub(crate) right: u32,
    pub(crate) bottom: u32,
}
impl SpacialHashRange {
    pub(crate) fn new(visible_section: Section, alignment: f32) -> Self {
        let left = (visible_section.left() / alignment).floor() as u32;
        let top = (visible_section.top() / alignment).floor() as u32;
        let right = (visible_section.right() / alignment).ceil() as u32;
        let bottom = (visible_section.bottom() / alignment).ceil() as u32;
        Self {
            left,
            top,
            right,
            bottom,
        }
    }
    pub(crate) fn hashes(&self) -> HashSet<SpacialHash> {
        let mut hashes = HashSet::new();
        for x in self.left..self.right {
            for y in self.top..self.bottom {
                hashes.insert(SpacialHash::new(x, y));
            }
        }
        hashes
    }
}
#[derive(Resource)]
pub(crate) struct SpacialHasher {
    pub(crate) alignment: f32,
    pub(crate) cached_hash_range: SpacialHashRange,
    pub(crate) entities: HashMap<SpacialHash, HashSet<Entity>>,
}

impl SpacialHasher {
    pub(crate) fn new(alignment: f32, visible_section: Section) -> Self {
        Self {
            alignment,
            cached_hash_range: SpacialHashRange::new(visible_section, alignment),
            entities: HashMap::new(),
        }
    }
    pub(crate) fn current_range(&self, visible_section: Section) -> SpacialHashRange {
        SpacialHashRange::new(visible_section, self.alignment)
    }
}
pub(crate) fn update_spacial_hash(
    mut spacial_hasher: ResMut<SpacialHasher>,
    mut changed: Query<
        (Entity, &Position, &Area, &mut SpacialHashCache),
        Or<(Changed<Position>, Changed<Area>)>,
    >,
) {
    let mut deferred_add = HashSet::<(Entity, SpacialHash)>::new();
    let mut added_hash_regions = HashSet::<SpacialHash>::new();
    for (entity, position, area, mut spacial_hash_cache) in changed.iter_mut() {
        let section: Section = (*position, *area).into();
        let current_range = spacial_hasher.current_range(section);
        let hashes = current_range.hashes();
        let removed = spacial_hash_cache
            .hashes
            .difference(&hashes)
            .copied()
            .collect::<HashSet<SpacialHash>>();
        for hash in removed {
            spacial_hasher
                .entities
                .get_mut(&hash)
                .expect("no entity set")
                .remove(&entity);
            spacial_hash_cache.hashes.remove(&hash);
        }
        let added = hashes
            .difference(&spacial_hash_cache.hashes)
            .copied()
            .collect::<HashSet<SpacialHash>>();
        for hash in added {
            deferred_add.insert((entity, hash));
            added_hash_regions.insert(hash);
            spacial_hash_cache.hashes.insert(hash);
        }
    }
    for added_region in added_hash_regions {
        spacial_hasher.entities.insert(added_region, HashSet::new());
    }
    for (entity, hash) in deferred_add {
        spacial_hasher
            .entities
            .get_mut(&hash)
            .expect("no entity set")
            .insert(entity);
    }
}
#[derive(Component)]
pub(crate) struct SpacialHashCache {
    pub(crate) hashes: HashSet<SpacialHash>,
}
impl SpacialHashCache {
    pub(crate) fn new() -> Self {
        Self {
            hashes: HashSet::new(),
        }
    }
}
pub(crate) fn calc_visible_section(
    visible_bounds: Res<VisibleBounds>,
    mut entities: Query<(
        Entity,
        &Position,
        &Area,
        &mut Visibility,
        Option<&mut VisibleSection>,
    )>,
    mut spacial_hasher: ResMut<SpacialHasher>,
    mut cmd: Commands,
) {
    let current_range = spacial_hasher.current_range(visible_bounds.section);
    let cached_hashes = spacial_hasher.cached_hash_range.hashes();
    let current_hashes = current_range.hashes();
    let removed_hashes = cached_hashes.difference(&current_hashes);
    let mut entity_remove_queue = HashSet::<Entity>::new();
    for hash in removed_hashes {
        if let Some(entity_set) = spacial_hasher.entities.get(hash) {
            for entity in entity_set.iter() {
                entity_remove_queue.insert(*entity);
            }
        }
    }
    for hash in current_hashes.iter() {
        if let Some(entity_set) = spacial_hasher.entities.get(hash) {
            for entity in entity_set {
                let (_entity, position, area, mut visibility, mut maybe_visible_section) =
                    entities.get_mut(*entity).expect("no entity found");
                if !visibility.visible() {
                    visibility.visible = true;
                }
                let section: Section = (*position, *area).into();
                let current_visible_section = section.intersection(visible_bounds.section);
                if let Some(mut visible_section) = maybe_visible_section {
                    if visible_section.section != current_visible_section {
                        *visible_section = VisibleSection::new(current_visible_section);
                    }
                } else {
                    cmd.entity(*entity).insert(current_visible_section);
                }
                entity_remove_queue.remove(entity);
            }
        }
    }
    for entity in entity_remove_queue {
        entities
            .get_mut(entity)
            .expect("entity not alive any longer")
            .3
            .visible = false;
        cmd.entity(entity).remove::<VisibleSection>();
    }
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
        let visible_section = ((0u32, 0u32), area.to_area(scale_factor)).into();
        stove
            .frontend
            .container
            .insert_resource(VisibleBounds::new(visible_section));
        stove
            .frontend
            .container
            .insert_resource(SpacialHasher::new(500f32, visible_section));
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
            update_spacial_hash
                .label(VisibilitySystems::UpdateSpacialHash)
                .after(VisibilitySystems::AdjustPosition),
        );
        stove.frontend.main.add_system_to_stage(
            FrontEndStages::ResolveVisibility,
            calc_visible_section.after(VisibilitySystems::UpdateSpacialHash),
        );
    }
}

#[derive(SystemLabel)]
pub enum VisibilitySystems {
    AdjustPosition,
    UpdateSpacialHash,
}
