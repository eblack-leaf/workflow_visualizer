use crate::viewport::ViewportHandle;
use crate::{Area, Attach, Engen, InterfaceContext, Position, Section};
use bevy_ecs::prelude::{Bundle, Component, Query, Res};
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
