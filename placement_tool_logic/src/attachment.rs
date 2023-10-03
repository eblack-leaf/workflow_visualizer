use crate::node_panel::{list_management, node_panel, process_triggers_node_panel};
use crate::resources::requests;
use workflow_visualizer::bevy_ecs::prelude::IntoSystemConfigs;
use workflow_visualizer::{Attach, SyncPoint, Visualizer};

pub struct LogicAttachment;
impl Attach for LogicAttachment {
    fn attach(visualizer: &mut Visualizer) {
        requests(visualizer);
        visualizer
            .task(Visualizer::TASK_STARTUP)
            .add_systems((node_panel.in_set(SyncPoint::PostInitialization),));
        visualizer.task(Visualizer::TASK_MAIN).add_systems((
            process_triggers_node_panel.in_set(SyncPoint::Process),
            list_management
                .in_set(SyncPoint::Process)
                .after(process_triggers_node_panel),
        ));
    }
}
