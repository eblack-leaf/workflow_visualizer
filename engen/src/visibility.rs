use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{Changed, Or, Query, Res, Resource};

use crate::{Area, Canvas, Position, Section, Task};
use crate::render::Extract;

#[derive(Component, Copy, Clone)]
pub(crate) struct Visibility {
    visible: bool,
    // separate cached/dirty check into WriteTracker<Visibility>
    cached_visibility: bool,
}

impl Visibility {
    pub(crate) fn new() -> Self {
        Self { visible: false, cached_visibility: false }
    }
    pub fn visibility_changed(&self) -> bool {
        self.visible != self.cached_visibility
    }
    pub fn set(&mut self, value: bool) {
        self.visible = value;
    }
    pub fn update_cached(&mut self) {
        self.cached_visibility = self.visible
    }
    pub fn visible(&self) -> bool {
        self.visible
    }
}

pub(crate) fn update_visibility_cache(mut entities: Query<(Entity, &mut Visibility)>) {
    for (entity, mut visibility) in entities.iter_mut() {
        visibility.update_cached();
    }
}

pub(crate) fn visibility(
    mut entities: Query<
        (Entity, &Position, Option<&Area>, &mut Visibility),
        Or<(Changed<Position>, Changed<Area>)>,
    >,
    viewport_bounds: Res<ViewportBounds>,
) {
    for (entity, position, maybe_area, mut visibility) in entities.iter_mut() {
        // if in viewport_bounds
        visibility.set(true);
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
        let viewport_bounds = compute.container.get_resource::<ViewportBounds>().expect("no viewport bounds");
        render.container.get_resource_mut::<Canvas>().expect("no canvas attached").update_viewport_offset(viewport_bounds.section.position);
    }
}