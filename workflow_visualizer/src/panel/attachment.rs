use bevy_ecs::prelude::IntoSystemConfig;

use crate::panel::renderer::PanelRenderer;
use crate::panel::system::{
    calc_area_from_content_area, color_diff, content_area_diff, layer_diff, management,
    position_diff, process_extraction, pull_differences,
};
use crate::panel::{renderer, Extraction};
use crate::{Attach, Engen, SyncPoint};

pub struct PanelAttachment;
impl Attach for PanelAttachment {
    fn attach(engen: &mut Engen) {
        engen.frontend.container.insert_resource(Extraction::new());
        engen.backend.container.insert_resource(Extraction::new());
        engen.add_renderer::<PanelRenderer>();
        engen
            .backend
            .startup
            .add_system(renderer::setup.in_set(SyncPoint::Preparation));
        engen
            .backend
            .main
            .add_system(process_extraction.in_set(SyncPoint::Preparation));
        engen.frontend.main.add_systems((
            calc_area_from_content_area.in_set(SyncPoint::Reconfigure),
            management.in_set(SyncPoint::Resolve),
            position_diff.in_set(SyncPoint::PushDiff),
            content_area_diff.in_set(SyncPoint::PushDiff),
            layer_diff.in_set(SyncPoint::PushDiff),
            color_diff.in_set(SyncPoint::PushDiff),
            pull_differences.in_set(SyncPoint::Finish),
        ));
    }
}
