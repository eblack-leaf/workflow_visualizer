use bevy_ecs::prelude::{Component, Query, Res};
use crate::{Area, Attach, Engen, InterfaceContext, Position, Section};
use crate::viewport::ViewportHandle;

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

#[derive(Component, Copy, Clone)]
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
    mut potentially_visible: Query<(&Position<InterfaceContext>, &Area<InterfaceContext>, &mut Visibility, &mut VisibleSection)>,
    viewport_handle: Res<ViewportHandle>,
) {
    for (pos, area, vis, vis_sec) in potentially_visible.iter_mut() {
        let section = Section::from((*pos, *area));
        let intersection = viewport_handle.section.intersection(section);
        let visible = intersection.is_some();
        if let Some(inter) = intersection {
            if let Some(current_vis_sec) = vis_sec.section.as_mut() {
                if current_vis_sec != inter {
                    *current_vis_sec = inter;
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
    }
}
pub struct VisibilityAttachment;
impl Attach for VisibilityAttachment {
    fn attach(engen: &mut Engen) {
        engen.frontend.main.add_system(calc_visibility);
    }
}