use std::collections::{HashMap, HashSet};

use bevy_ecs::change_detection::ResMut;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{Changed, Commands, Or, Query, Res, Resource};

use crate::visibility::collision::{Collision, CurrentOverlaps};
use crate::visibility::visible_bounds::VisibleBounds;
use crate::{Area, Position, Section, UIView, Visibility, VisibleSection};

#[derive(Resource)]
pub(crate) struct SpacialHasher {
    pub(crate) alignment: f32,
    pub(crate) cached_hash_range: SpacialHashRange,
    pub(crate) entities: HashMap<SpacialHash, HashSet<Entity>>,
    spacial_hash_cache: HashMap<Entity, HashSet<SpacialHash>>,
    pub(crate) current_overlaps: HashMap<Entity, CurrentOverlaps>,
    pub(crate) overlap_check_queue: HashSet<(Entity, SpacialHash)>,
    pub(crate) an_entity_changed: bool,
    pub(crate) visible_bounds_changed: bool,
}

impl SpacialHasher {
    pub(crate) fn new(alignment: f32, visible_section: Section<UIView>) -> Self {
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
    pub(crate) fn current_range(&self, visible_section: Section<UIView>) -> SpacialHashRange {
        SpacialHashRange::new(visible_section, self.alignment)
    }
    pub(crate) fn setup(&mut self, entity: Entity) {
        self.spacial_hash_cache.insert(entity, HashSet::new());
        self.current_overlaps.insert(entity, CurrentOverlaps::new());
    }
    pub(crate) fn cleanup(&mut self, entity: Entity) -> HashSet<Entity> {
        // self.overlap_check_queue.retain(|oc| oc.0 != entity); // elided but here until tested
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
        (
            Entity,
            &Position<UIView>,
            &Area<UIView>,
            &mut Visibility,
            Option<&Collision>,
        ),
        Or<(Changed<Position<UIView>>, Changed<Area<UIView>>)>,
    >,
    visible_bounds: Res<VisibleBounds>,
    mut cmd: Commands,
) {
    let mut deferred_add = HashSet::<(Entity, SpacialHash)>::new();
    let mut added_hash_regions = HashSet::<SpacialHash>::new();
    for (entity, position, area, mut visibility, maybe_collision) in changed.iter_mut() {
        spacial_hasher.an_entity_changed = true;
        let section: Section<UIView> = (*position, *area).into();
        if !section.is_overlapping(visible_bounds.section) {
            if visibility.visible() {
                visibility.visible = false;
                cmd.entity(entity).remove::<VisibleSection>();
            }
        }
        let current_range = spacial_hasher.current_range(section);
        let current_hashes = current_range.hashes();
        for hash in current_hashes.iter() {
            if maybe_collision.is_some() {
                spacial_hasher.overlap_check_queue.insert((entity, *hash));
            }
        }
        let removed = spacial_hasher
            .spacial_hash_cache
            .get_mut(&entity)
            .expect("spacial hash cache not setup")
            .difference(&current_hashes)
            .copied()
            .collect::<HashSet<SpacialHash>>();
        for hash in removed {
            if maybe_collision.is_some() {
                spacial_hasher.overlap_check_queue.insert((entity, hash));
            }
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
    pub(crate) fn new(visible_section: Section<UIView>, alignment: f32) -> Self {
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
