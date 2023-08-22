use bevy_ecs::change_detection::Res;
use bevy_ecs::prelude::{Commands, IntoSystemConfigs};

use crate::grid::system;
use crate::{Attach, Grid, SyncPoint, ViewportHandle, Visualizer};

pub(crate) struct GridAttachment;

impl Attach for GridAttachment {
    fn attach(visualizer: &mut Visualizer) {
        visualizer.job.task(Visualizer::TASK_STARTUP).add_systems((
            setup.in_set(SyncPoint::Initialization),
            system::config_grid.in_set(SyncPoint::PostInitialization),
        ));
        visualizer.job.task(Visualizer::TASK_MAIN).add_systems((
            system::set_from_absolute.in_set(SyncPoint::PostProcessPreparation),
            system::set_from_view.in_set(SyncPoint::PostProcessPreparation),
            system::set_point_from_view.in_set(SyncPoint::PostProcessPreparation),
        ));
    }
}

pub(crate) fn setup(viewport_handle: Res<ViewportHandle>, mut cmd: Commands) {
    let area = viewport_handle.section.area;
    let grid = Grid::new(area);
    cmd.insert_resource(grid);
}
