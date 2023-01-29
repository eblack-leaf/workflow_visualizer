use std::collections::HashSet;

use bevy_ecs::prelude::{
    Changed, Commands, Component, Entity, Or, Query, Res, ResMut, Resource, SystemStage,
};
use winit::window::Window;

use crate::canvas::Canvas;
use crate::coord::Position;
use crate::coord::{Area, ScaledArea};
use crate::extract::Extract;
use crate::task::Stage;
use crate::viewport::Viewport;
use crate::{Attach, Engen, Section, Task};

#[derive(Component)]
pub(crate) struct Visibility {
    visible: bool,
}

impl Visibility {
    pub(crate) fn new() -> Self {
        Self { visible: false }
    }
    pub(crate) fn visible(&self) -> bool {
        self.visible
    }
}

#[derive(Resource)]
pub(crate) struct ScaleFactor {
    pub(crate) factor: f64,
}

impl ScaleFactor {
    pub(crate) fn new(factor: f64) -> Self {
        Self { factor }
    }
}

impl From<f64> for ScaleFactor {
    fn from(value: f64) -> Self {
        Self::new(value)
    }
}

pub(crate) fn visibility(
    mut entities: Query<
        (Entity, &Position, Option<&Area>, &mut Visibility),
        Or<(Changed<Position>, Changed<Area>)>,
    >,
    visible_bounds: Res<VisibleBounds>,
) {
    for (entity, position, maybe_area, mut visibility) in entities.iter_mut() {
        // update spacial hash in visible entities
        let mut section = None;
        if let Some(area) = maybe_area {
            section.replace(Section::new(*position, *area));
        }
        if let Some(sec) = section {
            if sec.is_overlapping(visible_bounds.section) {
                if !visibility.visible() {
                    visibility.visible = true;
                }
            } else {
                if visibility.visible() {
                    visibility.visible = false;
                }
            }
        } else {
            if visible_bounds.section.contains(*position) {
                if !visibility.visible() {
                    visibility.visible = true;
                }
            } else {
                if visibility.visible() {
                    visibility.visible = false;
                }
            }
        }
    }
}

#[derive(Resource)]
pub(crate) struct VisibleBounds {
    section: Section,
    cache: Section,
    dirty: bool,
}

impl VisibleBounds {
    pub(crate) fn new(section: Section) -> Self {
        Self {
            section,
            cache: section,
            dirty: false,
        }
    }
    pub(crate) fn section(&self) -> &Section {
        &self.section
    }
    pub(crate) fn adjust(&mut self) -> &mut Section {
        self.dirty = true;
        &mut self.section
    }
}

#[derive(Resource)]
pub(crate) struct SpacialHasher {}

impl SpacialHasher {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

#[derive(Component)]
pub(crate) struct VisibilityChanged {
    pub(crate) new_visibility: bool,
}

impl VisibilityChanged {
    pub(crate) fn new(visibility: bool) -> Self {
        Self {
            new_visibility: visibility,
        }
    }
}

pub(crate) fn update_spacial_hash(mut visible_bounds: ResMut<VisibleBounds>, mut cmd: Commands) {
    if visible_bounds.dirty && visible_bounds.section != visible_bounds.cache {
        // check spacial hash of new place vs old place and
        // send message that visibility.visible changed for entity stored in spacial hasher
        visible_bounds.cache = visible_bounds.section;
    }
}

pub(crate) fn integrate_spacial_hash_changes(
    mut text: Query<(Entity, &mut Visibility, &VisibilityChanged), ()>,
    mut cmd: Commands,
) {
    for (entity, mut visibility, visibility_changed) in text.iter_mut() {
        if visibility.visible() && visibility_changed.new_visibility == false {
            visibility.visible = false;
        }
        if !visibility.visible() && visibility_changed.new_visibility == true {
            visibility.visible = true;
        }
        cmd.entity(entity).remove::<VisibilityChanged>();
    }
}

impl Extract for VisibleBounds {
    fn extract(frontend: &mut Task, backend: &mut Task) {
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
            let mut viewport = backend
                .container
                .remove_resource::<Viewport>()
                .expect("no viewport");
            let canvas = backend
                .container
                .get_resource::<Canvas>()
                .expect("no canvas");
            viewport.update_offset(
                &canvas.queue,
                visible_bounds.section.position.to_scaled(scale_factor),
            );
            backend.container.insert_resource(viewport);
            visible_bounds.dirty = false;
        }
    }
}
