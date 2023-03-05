use std::collections::HashSet;

use bevy_ecs::change_detection::ResMut;
use bevy_ecs::entity::Entity;
use bevy_ecs::event::EventReader;
use bevy_ecs::prelude::{Added, Commands, Query, RemovedComponents, Res};

use crate::visibility::spacial_hasher::SpacialHasher;
use crate::visibility::{Collision, CollisionBegin, CollisionEnd};
use crate::{
    Area, Position, Resize, ScaleFactor, Section, UIView, Visibility, VisibleBounds, VisibleSection,
};

pub(crate) fn visibility_setup(
    added: Query<Entity, Added<Visibility>>,
    added_collision: Query<Entity, Added<Collision>>,
    mut spacial_hasher: ResMut<SpacialHasher>,
    mut cmd: Commands,
) {
    for entity in added_collision.iter() {
        cmd.entity(entity)
            .insert((CollisionBegin::new(), CollisionEnd::new()));
    }
    for entity in added.iter() {
        spacial_hasher.setup(entity);
    }
}

pub(crate) fn visibility_cleanup(
    lost_visibility: RemovedComponents<Visibility>,
    mut spacial_hasher: ResMut<SpacialHasher>,
    mut lost_contact_entities: Query<&mut CollisionEnd>,
) {
    let mut other_removal = HashSet::new();
    let mut lost = HashSet::new();
    for entity in lost_visibility.iter() {
        lost.insert(entity);
    }
    for lost_entity in lost {
        let lost_contact = spacial_hasher.cleanup(lost_entity);
        for other in lost_contact {
            other_removal.insert((lost_entity, other));
        }
    }
    for (entity, other) in other_removal {
        match lost_contact_entities.get_mut(entity) {
            Ok(mut collision_end) => {
                collision_end.others.insert(other);
            }
            Err(_) => {}
        }
    }
}

pub(crate) fn calc_visible_section(
    visible_bounds: Res<VisibleBounds>,
    mut entities: Query<(
        Entity,
        &Position<UIView>,
        &Area<UIView>,
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
            let section: Section<UIView> = (*position, *area).into();
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

pub(crate) fn resize(
    mut resize_events: EventReader<Resize>,
    mut visible_bounds: ResMut<VisibleBounds>,
    scale_factor: Res<ScaleFactor>,
    mut spacial_hasher: ResMut<SpacialHasher>,
) {
    for event in resize_events.iter() {
        visible_bounds.adjust_area(event.size.to_ui(scale_factor.factor));
        spacial_hasher.visible_bounds_changed = true;
    }
}
