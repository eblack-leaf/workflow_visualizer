use crate::grid::{config_grid, HorizontalSpan};
use crate::{Area, Attach, Grid, GridLocation, InterfaceContext, Position, RawMarker, ResponsiveView, SyncPoint, Visualizer};
use bevy_ecs::prelude::{Changed, Component, DetectChanges, IntoSystemConfig, Query, Res};
use std::collections::HashMap;

#[derive(Component, Clone)]
pub struct Path {
    pub points: Vec<Position<InterfaceContext>>,
}
#[derive(Copy, Clone)]
pub struct PathViewPoint {
    pub x: GridLocation,
    pub y: GridLocation,
}
#[derive(Clone, Component)]
pub struct PathView {
    pub points: Vec<PathViewPoint>,
}
pub type ResponsivePathView = ResponsiveView<PathView>;
impl<T: Into<PathView>> From<T> for ResponsivePathView {
    fn from(value: T) -> Self {
        let value = value.into();
        let mut mapping = HashMap::new();
        mapping.insert(HorizontalSpan::Four, value.clone());
        mapping.insert(HorizontalSpan::Eight, value.clone());
        mapping.insert(HorizontalSpan::Twelve, value);
        ResponsivePathView { mapping }
    }
}
impl<T: Into<PathView>> From<(T, T, T)> for ResponsivePathView {
    fn from(value: (T, T, T)) -> Self {
        let mut mapping = HashMap::new();
        mapping.insert(HorizontalSpan::Four, value.0.into());
        mapping.insert(HorizontalSpan::Eight, value.1.into());
        mapping.insert(HorizontalSpan::Twelve, value.2.into());
        ResponsivePathView { mapping }
    }
}
impl ResponsivePathView {
    pub fn with_span_four<T: Into<PathView>>(mut self, view: T) -> Self {
        self.mapping.insert(HorizontalSpan::Four, view.into());
        self
    }
    pub fn with_span_eight<T: Into<PathView>>(mut self, view: T) -> Self {
        self.mapping.insert(HorizontalSpan::Eight, view.into());
        self
    }
    pub fn with_span_twelve<T: Into<PathView>>(mut self, view: T) -> Self {
        self.mapping.insert(HorizontalSpan::Twelve, view.into());
        self
    }
}
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
        path.points.push((x, y).into());
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
