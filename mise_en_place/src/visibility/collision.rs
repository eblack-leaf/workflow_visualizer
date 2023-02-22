use std::collections::HashSet;

use bevy_ecs::change_detection::ResMut;
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{Changed, Or, Query, With};

use crate::visibility::spacial_hasher::SpacialHasher;
use crate::{Area, Position, Section, UIView, Visibility};

pub(crate) fn collision_responses(
    mut spacial_hasher: ResMut<SpacialHasher>,
    mut entities: Query<
        (
            &Position<UIView>,
            &Area<UIView>,
            &mut CollisionBegin,
            &mut CollisionEnd,
        ),
        (With<Visibility>, With<Collision>),
    >,
) {
    let mut checks = HashSet::new();
    for (entity, hash) in spacial_hasher.overlap_check_queue.iter() {
        let others = spacial_hasher.entities.get(hash).expect("no entity set");
        for other in others {
            if *entity != *other {
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
    }
    spacial_hasher.overlap_check_queue.clear();
    let mut overlap_adds = HashSet::<(Entity, Entity)>::new();
    let mut overlap_removes = HashSet::<(Entity, Entity)>::new();
    for (lhs, rhs) in checks {
        if let Ok((position, area, _, _)) = entities.get(lhs) {
            if let Ok((other_position, other_area, _, _)) = entities.get(rhs) {
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

#[derive(Component, Copy, Clone)]
pub struct Collision {}

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
