use crate::content_panel::renderer;
use crate::content_panel::renderer::ContentPanelRenderer;
use crate::content_panel::system::{
    calc_area_from_content_area, color_diff, content_area_diff, layer_diff, position_diff,
    pull_differences,
};
use crate::{Attach, Engen, SyncPoint};
use bevy_ecs::prelude::IntoSystemConfig;

pub struct ContentPanelAttachment;
impl Attach for ContentPanelAttachment {
    fn attach(engen: &mut Engen) {
        engen.add_renderer::<ContentPanelRenderer>();
        engen
            .backend
            .startup
            .add_system(renderer::setup.in_set(SyncPoint::Preparation));
        engen
            .frontend
            .main
            .add_system(calc_area_from_content_area.in_set(SyncPoint::Reconfigure));
        engen.frontend.main.add_systems(
            (position_diff.in_set(SyncPoint::PushDiff), content_area_diff.in_set(SyncPoint::PushDiff), layer_diff.in_set(SyncPoint::PushDiff), color_diff.in_set(SyncPoint::PushDiff)),
        );
        engen
            .frontend
            .main
            .add_system(pull_differences.in_set(SyncPoint::Finish));
    }
}
