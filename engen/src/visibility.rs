use std::collections::HashSet;

use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{Changed, Or, Query, Res, ResMut, Resource};

use crate::coord::{Movement, Scale};
use crate::render::Extract;
use crate::{Area, Canvas, Position, Section, Task};

#[derive(Component, Copy, Clone)]
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

pub(crate) fn visibility(
    mut entities: Query<
        (Entity, &Position, Option<&Area>, &mut Visibility),
        Or<(Changed<Position>, Changed<Area>)>,
    >,
    viewport_bounds: Res<ViewportBounds>,
    visible_entities: ResMut<VisibleEntities>,
) {
    for (entity, position, maybe_area, mut visibility) in entities.iter_mut() {
        // update spacial hash in visible entities
        // else not in bounds && in visible entities
        visibility.visible = false;
        // if in viewport_bounds && not in visible entities
        visibility.visible = true;
    }
}

#[derive(Resource)]
pub(crate) struct ViewportBounds {
    pub(crate) section: Section,
    pub(crate) dirty: bool,
}

impl ViewportBounds {
    pub(crate) fn new(section: Section) -> Self {
        Self {
            section,
            dirty: true,
        }
    }
    pub(crate) fn apply(&mut self, movement: Movement) {
        self.section.position.apply(movement);
        self.dirty = true;
    }
    pub(crate) fn resize(&mut self, scale: Scale) {
        self.section.area.apply(scale);
    }
}

#[derive(Resource)]
pub(crate) struct ViewportBoundsScale {
    pub(crate) scale: Option<Scale>,
}

impl ViewportBoundsScale {
    pub(crate) fn new() -> Self {
        Self { scale: None }
    }
}

#[derive(Resource)]
pub(crate) struct ViewportBoundsMovement {
    pub(crate) movement: Option<Movement>,
}

impl ViewportBoundsMovement {
    pub(crate) fn new() -> Self {
        Self { movement: None }
    }
}

pub(crate) fn move_viewport_bounds(
    mut viewport_bounds: ResMut<ViewportBounds>,
    mut viewport_bounds_movement: ResMut<ViewportBoundsMovement>,
    mut viewport_bounds_scale: ResMut<ViewportBoundsScale>,
) {
    // take old spacial hash and remove all entities in those nodes
    if let Some(movement) = viewport_bounds_movement.movement.take() {
        viewport_bounds.apply(movement);
    }
    if let Some(scale) = viewport_bounds_scale.scale.take() {
        viewport_bounds.resize(scale);
    }
    // take new spacial hash and add all entities to visible_entities_update
}

impl Extract for ViewportBounds {
    fn extract(compute: &mut Task, render: &mut Task)
    where
        Self: Sized,
    {
        let mut viewport_bounds = compute
            .container
            .get_resource_mut::<ViewportBounds>()
            .expect("no viewport bounds");
        if viewport_bounds.dirty {
            render
                .container
                .get_resource_mut::<Canvas>()
                .expect("no canvas attached")
                .update_viewport_offset(viewport_bounds.section.position);
            viewport_bounds.dirty = false;
        }
    }
}

#[derive(Resource)]
pub(crate) struct VisibleEntities {
    pub(crate) visible_cache: HashSet<Entity>,
}

impl VisibleEntities {
    pub(crate) fn new() -> Self {
        Self {
            visible_cache: HashSet::new(),
        }
    }
}
