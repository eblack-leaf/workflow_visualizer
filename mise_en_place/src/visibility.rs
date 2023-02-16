use std::collections::{HashMap, HashSet};

use bevy_ecs::prelude::{
    Added, Changed, Commands, Component, Entity, EventReader, IntoSystemDescriptor, Or, Query,
    RemovedComponents, Res, ResMut, Resource, SystemLabel, With, Without,
};

use crate::coord::{Area, Device, Position, PositionAdjust, Section, View};
use crate::extract::Extract;
use crate::gfx::{GfxSurface, GfxSurfaceConfiguration};
use crate::viewport::Viewport;
use crate::window::{Resize, ScaleFactor};
use crate::{Attach, BackendStages, Engen, FrontEndStages, Job};

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
    pub(crate) section: Section<View>,
}

impl VisibleSection {
    pub(crate) fn new(section: Section<View>) -> Self {
        Self { section }
    }
    pub fn section(&self) -> Section<View> {
        self.section
    }
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
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
    pub(crate) fn new(visible_section: Section<View>, alignment: f32) -> Self {
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
    spacial_hash_cache: HashMap<Entity, HashSet<SpacialHash>>,
    pub current_overlaps: HashMap<Entity, CurrentOverlaps>,
    overlap_check_queue: HashSet<(Entity, SpacialHash)>,
    an_entity_changed: bool,
    visible_bounds_changed: bool,
}

impl SpacialHasher {
    pub(crate) fn new(alignment: f32, visible_section: Section<View>) -> Self {
        Self {
            alignment,
            cached_hash_range: SpacialHashRange::new(visible_section, alignment),
            entities: HashMap::new(),
            spacial_hash_cache: HashMap::new(),
            current_overlaps: HashMap::new(),
            overlap_check_queue: HashSet::new(),
            an_entity_changed: false,
            visible_bounds_changed: false,
        }
    }
    pub(crate) fn current_range(&self, visible_section: Section<View>) -> SpacialHashRange {
        SpacialHashRange::new(visible_section, self.alignment)
    }
    fn setup(&mut self, entity: Entity) {
        self.spacial_hash_cache.insert(entity, HashSet::new());
        self.current_overlaps.insert(entity, CurrentOverlaps::new());
    }
    fn cleanup(&mut self, entity: Entity) -> HashSet<Entity> {
        let old = self.spacial_hash_cache.remove(&entity);
        if let Some(hashes) = old {
            for hash in hashes {
                self.entities
                    .get_mut(&hash)
                    .expect("entity set not setup")
                    .remove(&entity);
            }
        }
        let old = self.current_overlaps.remove(&entity);
        let mut response = HashSet::new();
        if let Some(overlaps) = old {
            for other in overlaps.entities {
                self.current_overlaps
                    .get_mut(&other)
                    .expect("no overlaps")
                    .entities
                    .remove(&entity);
                response.insert(entity);
            }
        }
        response
    }
}

pub(crate) fn update_spacial_hash(
    mut spacial_hasher: ResMut<SpacialHasher>,
    mut changed: Query<
        (Entity, &Position<View>, &Area<View>, &mut Visibility),
        Or<(Changed<Position<View>>, Changed<Area<View>>)>,
    >,
    visible_bounds: Res<VisibleBounds>,
    mut cmd: Commands,
) {
    let mut deferred_add = HashSet::<(Entity, SpacialHash)>::new();
    let mut added_hash_regions = HashSet::<SpacialHash>::new();
    for (entity, position, area, mut visibility) in changed.iter_mut() {
        spacial_hasher.an_entity_changed = true;
        let section: Section<View> = (View {}, *position, *area).into();
        if !section.is_overlapping(visible_bounds.section) {
            if visibility.visible() {
                visibility.visible = false;
                cmd.entity(entity).remove::<VisibleSection>();
            }
        }
        let current_range = spacial_hasher.current_range(section);
        let current_hashes = current_range.hashes();
        for hash in current_hashes.iter() {
            spacial_hasher.overlap_check_queue.insert((entity, *hash));
        }
        let removed = spacial_hasher
            .spacial_hash_cache
            .get_mut(&entity)
            .expect("spacial hash cache not setup")
            .difference(&current_hashes)
            .copied()
            .collect::<HashSet<SpacialHash>>();
        for hash in removed {
            spacial_hasher.overlap_check_queue.insert((entity, hash));
            spacial_hasher
                .entities
                .get_mut(&hash)
                .expect("no entity set")
                .remove(&entity);
            spacial_hasher
                .spacial_hash_cache
                .get_mut(&entity)
                .expect("spacial hash cache not setup")
                .remove(&hash);
        }

        let added = current_hashes
            .difference(
                &spacial_hasher
                    .spacial_hash_cache
                    .get(&entity)
                    .expect("spacial hash cache not setup"),
            )
            .copied()
            .collect::<HashSet<SpacialHash>>();
        for hash in added {
            deferred_add.insert((entity, hash));
            added_hash_regions.insert(hash);
            spacial_hasher
                .spacial_hash_cache
                .get_mut(&entity)
                .expect("spacial hash cache not setup")
                .insert(hash);
        }
    }
    for added_region in added_hash_regions {
        if !spacial_hasher.entities.contains_key(&added_region) {
            spacial_hasher.entities.insert(added_region, HashSet::new());
        }
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
pub struct CollisionBegin {
    pub others: HashSet<Entity>,
}

impl CollisionBegin {
    pub(crate) fn new() -> Self {
        Self {
            others: HashSet::new(),
        }
    }
}

#[derive(Component)]
pub struct CollisionEnd {
    pub others: HashSet<Entity>,
}

impl CollisionEnd {
    pub(crate) fn new() -> Self {
        Self {
            others: HashSet::new(),
        }
    }
}

pub(crate) fn collision_responses(
    mut spacial_hasher: ResMut<SpacialHasher>,
    mut entities: Query<
        (
            &Position<View>,
            &Area<View>,
            &mut CollisionBegin,
            &mut CollisionEnd,
        ),
        With<Visibility>,
    >,
) {
    let mut checks = HashSet::new();
    for (entity, hash) in spacial_hasher.overlap_check_queue.iter() {
        let others = spacial_hasher.entities.get(hash).expect("no entity set");
        for other in others {
            let mut smaller_index_entity = entity;
            let mut higher_index_entity = other;
            if entity.index() > other.index() {
                smaller_index_entity = other;
                higher_index_entity = entity;
            } else if entity.index() == other.index() {
                if entity.generation() > other.generation() {
                    smaller_index_entity = other;
                    higher_index_entity = entity;
                }
            }
            checks.insert((*smaller_index_entity, *higher_index_entity));
        }
    }
    let mut overlap_adds = HashSet::<(Entity, Entity)>::new();
    let mut overlap_removes = HashSet::<(Entity, Entity)>::new();
    for (lhs, rhs) in checks {
        let (position, area, _, _) = entities.get(lhs).expect("no entity for lhs");
        let (other_position, other_area, _, _) = entities.get(rhs).expect("no entity for rhs");
        let lhs_current_overlaps = spacial_hasher
            .current_overlaps
            .get(&lhs)
            .expect("no current overlaps");
        let rhs_current_overlaps = spacial_hasher
            .current_overlaps
            .get(&rhs)
            .expect("no current overlaps");
        let lhs_section = Section::new(*position, *area);
        let rhs_section = Section::new(*other_position, *other_area);
        if lhs_section.is_touching(rhs_section) {
            if !lhs_current_overlaps.entities.contains(&rhs) {
                overlap_adds.insert((lhs, rhs));
            }
            if !rhs_current_overlaps.entities.contains(&lhs) {
                overlap_adds.insert((rhs, lhs));
            }
        } else {
            if lhs_current_overlaps.entities.contains(&rhs) {
                overlap_removes.insert((lhs, rhs));
            }
            if rhs_current_overlaps.entities.contains(&lhs) {
                overlap_removes.insert((rhs, lhs));
            }
        }
    }
    for add in overlap_adds {
        spacial_hasher
            .current_overlaps
            .get_mut(&add.0)
            .expect("no current overlaps")
            .entities
            .insert(add.1);
        let (_, _, mut collision_begin, _) = entities.get_mut(add.0).expect("no entity");
        collision_begin.others.insert(add.1);
    }
    for remove in overlap_removes {
        spacial_hasher
            .current_overlaps
            .get_mut(&remove.0)
            .expect("no current overlaps")
            .entities
            .remove(&remove.1);
        let (_, _, _, mut collision_end) = entities.get_mut(remove.0).expect("no entity");
        collision_end.others.insert(remove.1);
    }
}

pub(crate) fn clean_collision_responses(
    mut entities: Query<
        (&mut CollisionBegin, &mut CollisionEnd),
        Or<(Changed<CollisionBegin>, Changed<CollisionEnd>)>,
    >,
) {
    for (mut collision_begin, mut collision_end) in entities.iter_mut() {
        collision_begin.others.clear();
        collision_end.others.clear();
    }
}

pub struct CurrentOverlaps {
    pub entities: HashSet<Entity>,
}

impl CurrentOverlaps {
    pub(crate) fn new() -> Self {
        Self {
            entities: HashSet::new(),
        }
    }
}

pub(crate) fn visibility_setup(
    added: Query<
        Entity,
        (
            Or<(Added<Position<View>>, Added<Area<View>>)>,
            With<Position<View>>,
            With<Area<View>>,
            Without<Visibility>,
        ),
    >,
    mut spacial_hasher: ResMut<SpacialHasher>,
    mut cmd: Commands,
) {
    for entity in added.iter() {
        cmd.entity(entity).insert((
            Visibility::new(),
            CollisionBegin::new(),
            CollisionEnd::new(),
        ));
        spacial_hasher.setup(entity);
    }
}

pub(crate) fn visibility_cleanup(
    lost_position: RemovedComponents<Position<View>>,
    lost_area: RemovedComponents<Area<View>>,
    lost_visibility: RemovedComponents<Visibility>,
    mut spacial_hasher: ResMut<SpacialHasher>,
    mut lost_contact_entities: Query<&mut CollisionEnd>,
    mut cmd: Commands,
) {
    for entity in lost_visibility.iter() {
        let lost_contact = spacial_hasher.cleanup(entity);
        for other in lost_contact {
            match lost_contact_entities.get_mut(entity) {
                Ok(mut collision_end) => {
                    collision_end.others.insert(other);
                }
                Err(_) => {}
            }
        }
        cmd.entity(entity).remove::<CollisionBegin>();
        cmd.entity(entity).remove::<CollisionEnd>();
        cmd.entity(entity).remove::<VisibleSection>();
    }
    for entity in lost_position.iter() {
        let lost_contact = spacial_hasher.cleanup(entity);
        for other in lost_contact {
            match lost_contact_entities.get_mut(entity) {
                Ok(mut collision_end) => {
                    collision_end.others.insert(other);
                }
                Err(_) => {}
            }
        }
        cmd.entity(entity).remove::<Visibility>();
        cmd.entity(entity).remove::<CollisionBegin>();
        cmd.entity(entity).remove::<CollisionEnd>();
        cmd.entity(entity).remove::<VisibleSection>();
    }
    for entity in lost_area.iter() {
        let lost_contact = spacial_hasher.cleanup(entity);
        for other in lost_contact {
            match lost_contact_entities.get_mut(entity) {
                Ok(mut collision_end) => {
                    collision_end.others.insert(other);
                }
                Err(_) => {}
            }
        }
        cmd.entity(entity).remove::<Visibility>();
        cmd.entity(entity).remove::<CollisionBegin>();
        cmd.entity(entity).remove::<CollisionEnd>();
        cmd.entity(entity).remove::<VisibleSection>();
    }
}

pub(crate) fn calc_visible_section(
    visible_bounds: Res<VisibleBounds>,
    mut entities: Query<(
        Entity,
        &Position<View>,
        &Area<View>,
        &mut Visibility,
        Option<&mut VisibleSection>,
    )>,
    mut spacial_hasher: ResMut<SpacialHasher>,
    mut cmd: Commands,
) {
    if spacial_hasher.visible_bounds_changed || spacial_hasher.an_entity_changed {
        spacial_hasher.visible_bounds_changed = false;
        spacial_hasher.an_entity_changed = false;
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
        let mut entities_to_check = HashSet::new();
        for hash in current_hashes.iter() {
            if let Some(entity_set) = spacial_hasher.entities.get(hash) {
                for entity in entity_set {
                    entities_to_check.insert(*entity);
                }
            }
        }
        for entity in entities_to_check {
            let (_entity, position, area, mut visibility, maybe_visible_section) =
                entities.get_mut(entity).expect("no entity found");
            let section: Section<View> = (View {}, *position, *area).into();
            if section.is_overlapping(visible_bounds.section) {
                if !visibility.visible() {
                    visibility.visible = true;
                }
                let current_visible_section = section.intersection(visible_bounds.section).unwrap();
                if let Some(mut visible_section) = maybe_visible_section {
                    if visible_section.section != current_visible_section {
                        *visible_section = VisibleSection::new(current_visible_section);
                    }
                } else {
                    cmd.entity(entity)
                        .insert(VisibleSection::new(current_visible_section));
                }
                entity_remove_queue.remove(&entity);
            } else if visibility.visible() {
                entity_remove_queue.insert(entity);
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
}

#[derive(Resource)]
pub struct VisibleBounds {
    pub(crate) section: Section<View>,
    dirty: bool,
}

impl VisibleBounds {
    pub(crate) fn new(section: Section<View>) -> Self {
        Self {
            section,
            dirty: false,
        }
    }
    pub fn position_adjust(&mut self, adjust: PositionAdjust<View>) {
        self.section.position.adjust(adjust);
        self.dirty = true;
    }
    pub fn adjust_area(&mut self, area: Area<View>) {
        self.section.area = area;
    }
}

#[derive(Resource)]
pub struct VisibleBoundsPositionAdjust {
    pub adjust: Option<PositionAdjust<View>>,
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
    pub(crate) update: Option<Position<Device>>,
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

pub(crate) fn resize(
    mut resize_events: EventReader<Resize>,
    mut visible_bounds: ResMut<VisibleBounds>,
    scale_factor: Res<ScaleFactor>,
    mut spacial_hasher: ResMut<SpacialHasher>,
) {
    for event in resize_events.iter() {
        visible_bounds.adjust_area(event.size.to_view(scale_factor.factor));
        spacial_hasher.visible_bounds_changed = true;
    }
}

impl Attach for Visibility {
    fn attach(engen: &mut Engen) {
        engen
            .backend
            .container
            .insert_resource(ViewportOffsetUpdate::new());
        engen.add_extraction::<VisibleBounds>();
        let gfx_surface_configuration = engen
            .backend
            .container
            .get_resource::<GfxSurfaceConfiguration>()
            .expect("no gfx surface config");
        let scale_factor = engen
            .frontend
            .container
            .get_resource::<ScaleFactor>()
            .expect("no scale factor")
            .factor;
        let surface_area: Area<Device> = (
            gfx_surface_configuration.configuration.width,
            gfx_surface_configuration.configuration.height,
        )
            .into();
        let visible_section = (View {}, (0u32, 0u32), surface_area.to_view(scale_factor)).into();
        engen
            .frontend
            .container
            .insert_resource(VisibleBounds::new(visible_section));
        engen
            .frontend
            .container
            .insert_resource(SpacialHasher::new(500f32, visible_section));
        engen
            .frontend
            .container
            .insert_resource(VisibleBoundsPositionAdjust::new());
        engen
            .backend
            .main
            .add_system_to_stage(BackendStages::Resize, viewport_read_offset);
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::Resize, resize);
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::VisibilityPreparation, visibility_setup);
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::VisibilityPreparation, visibility_cleanup);
        engen.frontend.main.add_system_to_stage(
            FrontEndStages::ResolveVisibility,
            adjust_position.label(VisibilitySystems::AdjustPosition),
        );
        engen.frontend.main.add_system_to_stage(
            FrontEndStages::ResolveVisibility,
            update_spacial_hash
                .label(VisibilitySystems::UpdateSpacialHash)
                .after(VisibilitySystems::AdjustPosition),
        );
        engen.frontend.main.add_system_to_stage(
            FrontEndStages::ResolveVisibility,
            collision_responses.after(VisibilitySystems::UpdateSpacialHash),
        );
        engen.frontend.main.add_system_to_stage(
            FrontEndStages::ResolveVisibility,
            calc_visible_section.after(VisibilitySystems::UpdateSpacialHash),
        );
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::Last, clean_collision_responses);
    }
}

#[derive(SystemLabel)]
enum VisibilitySystems {
    AdjustPosition,
    UpdateSpacialHash,
}
