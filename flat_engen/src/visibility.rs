use winit::window::Window;
use bevy_ecs::prelude::{Changed, Component, Entity, Or, Query, Resource, SystemStage};
use crate::{Attach, Engen};
use crate::coord::Area;
use crate::coord::Position;
use crate::task::Stage;
#[derive(Component)]
pub(crate) struct Visibility {
    visible: bool,
}
impl Visibility {
    pub(crate) fn new() -> Self {
        Self {
            visible: false,
        }
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
        Self {
            factor,
        }
    }
    pub(crate) fn attach(window: &Window, engen: &mut Engen) {
        engen.frontend.container.insert_resource(ScaleFactor::new(window.scale_factor()));
        engen.backend.container.insert_resource(ScaleFactor::new(window.scale_factor()));
    }
}

impl From<f64> for ScaleFactor {
    fn from(value: f64) -> Self {
        Self::new(value)
    }
}
impl Attach for Visibility {
    fn attach(engen: &mut Engen) {
        engen.frontend.main.schedule.add_stage_before(Stage::After, "visibility", SystemStage::single(visibility));
    }
}
pub(crate) fn visibility(
    mut entities: Query<
        (Entity, &Position, Option<&Area>, &mut Visibility),
        Or<(Changed<Position>, Changed<Area>)>,
    >,
    // viewport_bounds: Res<ViewportBounds>,
    // visible_entities: ResMut<VisibleEntities>,
) {
    for (entity, position, maybe_area, mut visibility) in entities.iter_mut() {
        // update spacial hash in visible entities
        // else not in bounds && in visible entities
        visibility.visible = false;
        // if in viewport_bounds && not in visible entities
        visibility.visible = true;
    }
}
