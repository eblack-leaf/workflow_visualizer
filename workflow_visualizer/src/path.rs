use crate::{Grid, InterfaceContext, Marker, Position, ResponsiveView};
use bevy_ecs::prelude::{Changed, Component, DetectChanges, Query, Res};
#[derive(Component)]
pub struct Path {
    pub points: Vec<Position<InterfaceContext>>,
}
pub struct PathView {
    pub points: Vec<Marker>,
}
pub type ResponsivePathView = ResponsiveView<PathView>;
pub(crate) fn grid_updated_path(
    grid: Res<Grid>,
    mut responsively_viewed: Query<(&ResponsiveView<PathView>, &mut Path)>,
) {
    if grid.is_changed() {
        for (view, mut path) in responsively_viewed.iter_mut() {}
    }
}
pub(crate) fn view_changed(
    mut responsively_viewed: Query<
        (&ResponsiveView<PathView>, &mut Path),
        Changed<ResponsiveView<PathView>>,
    >,
) {
    for (view, mut path) in responsively_viewed.iter_mut() {}
}
