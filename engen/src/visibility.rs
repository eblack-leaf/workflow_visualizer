use std::collections::HashSet;
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{Changed, Or, Query, Res, ResMut, Resource};

use crate::{Area, Canvas, Position, Section, Task};
use crate::render::Extract;

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
    // could forego query and logical grid hash of entities and calc changes checking only in hash grid objs
    // then for each difference - cmd.entity(entity).insert(update_visibility);
    for (entity, position, maybe_area, mut visibility) in entities.iter_mut() {
        // else not in bounds && in visible entities
        visibility.visible = false;
        // if in viewport_bounds && not in visible entities
        visibility.visible = true;
    }
}

#[derive(Resource)]
pub(crate) struct ViewportBounds {
    pub(crate) section: Section,
}

impl ViewportBounds {
    pub(crate) fn new(section: Section) -> Self {
        Self { section }
    }
}

impl Extract for ViewportBounds {
    fn extract(compute: &mut Task, render: &mut Task) where Self: Sized {
        // currently sending every frame - should cache
        let viewport_bounds = compute.container.get_resource::<ViewportBounds>().expect("no viewport bounds");
        render.container.get_resource_mut::<Canvas>().expect("no canvas attached").update_viewport_offset(viewport_bounds.section.position);
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