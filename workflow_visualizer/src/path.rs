use std::collections::HashMap;

use bevy_ecs::prelude::{Changed, Component, DetectChanges, IntoSystemConfig, Query, Res};

use crate::{
    Area, Attach, Grid, GridLocation, InterfaceContext, Position, RawMarker, ResponsiveView,
    SyncPoint, Visualizer,
};
use crate::grid::{config_grid, HorizontalSpan};

#[derive(Component, Clone)]
pub struct Path {
    pub points: Vec<Position<InterfaceContext>>,
}

impl Path {
    pub(crate) fn new(points: Vec<Position<InterfaceContext>>) -> Path {
        Self { points }
    }
}

#[derive(Copy, Clone)]
pub struct PathViewPoint {
    pub x: GridLocation,
    pub y: GridLocation,
}

impl<T: Into<GridLocation>> From<(T, T)> for PathViewPoint {
    fn from(value: (T, T)) -> Self {
        Self {
            x: value.0.into(),
            y: value.1.into(),
        }
    }
}
#[derive(Clone, Component)]
pub struct PathView {
    pub points: Vec<PathViewPoint>,
}

impl From<Vec<PathViewPoint>> for PathView {
    fn from(value: Vec<PathViewPoint>) -> Self {
        Self { points: value }
    }
}
pub type ResponsivePathView = ResponsiveView<PathView>;
pub(crate) fn grid_updated_path(
    grid: Res<Grid>,
    mut responsively_viewed: Query<(&ResponsiveView<PathView>, &mut Path)>,
) {
    if grid.is_changed() {
        for (view, mut path) in responsively_viewed.iter_mut() {
            update_path(&grid, view, &mut path);
        }
    }
}
pub(crate) fn view_changed(
    mut responsively_viewed: Query<
        (&ResponsiveView<PathView>, &mut Path),
        Changed<ResponsiveView<PathView>>,
    >,
    grid: Res<Grid>,
) {
    for (view, mut path) in responsively_viewed.iter_mut() {
        update_path(&grid, view, &mut path);
    }
}
fn update_path(grid: &Grid, view: &ResponsiveView<PathView>, path: &mut Path) {
    let current_view = view.mapping.get(&grid.span).expect("view mapping");
    path.points.clear();
    for point in current_view.points.iter() {
        let x = grid.calc_horizontal_location(point.x);
        let y = grid.calc_vertical_location(point.y);
        path.points.push((x.to_pixel(), y.to_pixel()).into());
    }
}
pub(crate) struct PathAttachment;
impl Attach for PathAttachment {
    fn attach(visualizer: &mut Visualizer) {
        visualizer.job.task(Visualizer::TASK_MAIN).add_systems((
            grid_updated_path
                .in_set(SyncPoint::Config)
                .after(config_grid),
            view_changed.in_set(SyncPoint::Reconfigure),
        ));
    }
}
