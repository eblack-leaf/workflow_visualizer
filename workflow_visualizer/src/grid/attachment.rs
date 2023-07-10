use bevy_ecs::change_detection::Res;
use bevy_ecs::prelude::{Commands, IntoSystemConfig};

use crate::{Attach, grid, Grid, SyncPoint, ViewportHandle, Visualizer};
use crate::grid::system;

pub(crate) struct GridAttachment;

impl Attach for GridAttachment {
    fn attach(visualizer: &mut Visualizer) {
        visualizer
            .job
            .task(Visualizer::TASK_STARTUP)
            .add_systems((setup.in_set(SyncPoint::Initialization), ));
        visualizer.job.task(Visualizer::TASK_MAIN).add_systems((
            system::config_grid.in_set(SyncPoint::Config),
            system::set_from_view.in_set(SyncPoint::Grid),
            system::set_point_from_view.in_set(SyncPoint::Grid),
        ));
    }
}

pub(crate) fn setup(viewport_handle: Res<ViewportHandle>, mut cmd: Commands) {
    let area = viewport_handle.section.area;
    let grid = Grid::new(area);
    cmd.insert_resource(grid);
}
