use bevy_ecs::prelude::{Bundle, Component, IntoSystemConfig, Query, Res};
use tracing::{trace, warn};

use crate::{Area, InterfaceContext, Position, Section, SyncPoint};
use crate::grid::config_grid;
use crate::path::grid_updated_path;
use crate::viewport::ViewportHandle;
use crate::visualizer::{Attach, Visualizer};

/// Entry point for enabling visibility module on an entity
#[derive(Bundle)]
pub struct EnableVisibility {
    pub visibility: Visibility,
    pub visible_section: VisibleSection,
}
impl EnableVisibility {
    pub fn new() -> Self {
        Self {
            visibility: Visibility::new(),
            visible_section: VisibleSection::new(None),
        }
    }
}
impl Default for EnableVisibility {
    fn default() -> Self {
        EnableVisibility::new()
    }
}
/// Current Visibility
#[derive(Component, Copy, Clone)]
pub struct Visibility {
    visible: bool,
}

impl Visibility {
    pub(crate) fn new() -> Self {
        Self { visible: false }
    }
    pub fn visible(&self) -> bool {
        self.visible
    }
}
/// What part of the entity is visible
#[derive(Component, Copy, Clone, Default)]
pub struct VisibleSection {
    pub(crate) section: Option<Section<InterfaceContext>>,
}

impl VisibleSection {
    pub(crate) fn new(section: Option<Section<InterfaceContext>>) -> Self {
        Self { section }
    }
    pub fn section(&self) -> Option<Section<InterfaceContext>> {
        self.section
    }
}
pub(crate) fn calc_visibility(
    mut potentially_visible: Query<(
        &Position<InterfaceContext>,
        &Area<InterfaceContext>,
        &mut Visibility,
        &mut VisibleSection,
    )>,
    viewport_handle: Res<ViewportHandle>,
) {
    for (pos, area, mut vis, mut vis_sec) in potentially_visible.iter_mut() {
        let section = Section::from((*pos, *area));
        let intersection = viewport_handle.section.intersection(section);
        let visible = intersection.is_some();
        if let Some(inter) = intersection {
            if let Some(current_vis_sec) = vis_sec.section {
                if current_vis_sec != inter {
                    vis_sec.section.replace(inter);
                }
            } else {
                vis_sec.section.replace(inter);
            }
        } else {
            vis_sec.section.take();
        }
        if vis.visible() != visible {
            vis.visible = visible;
        }
        trace!(
            "establishing visibility for {:?}, {:?}, {:?}",
            pos,
            area,
            vis.visible()
        );
    }
}
pub(crate) struct VisibilityAttachment;
impl Attach for VisibilityAttachment {
    fn attach(visualizer: &mut Visualizer) {
        visualizer.job.task(Visualizer::TASK_MAIN).add_systems((
            calc_visibility.in_set(SyncPoint::PreProcessVisibility),
            calc_visibility.in_set(SyncPoint::PostProcessVisibility),
        ));
    }
}
