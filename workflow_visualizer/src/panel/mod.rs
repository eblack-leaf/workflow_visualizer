use std::collections::{HashMap, HashSet};

use bevy_ecs::prelude::{Bundle, Component, Entity, Resource};

pub use attachment::PanelAttachment;
pub use system::calc_content_area;

use crate::{
    Area, Color, EnableVisibility, InterfaceContext, Layer, Position, ResponsiveGridView, Section,
};

mod attachment;
mod renderer;
mod system;
mod vertex;

#[derive(Component, Copy, Clone)]
pub struct PanelContentArea(pub Area<InterfaceContext>);
#[derive(Component, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum PanelType {
    Panel,
    Border,
    BorderedPanel,
}
#[derive(Component, Copy, Clone, PartialEq)]
pub struct PanelColor(pub Color);
#[derive(Component, Copy, Clone)]
pub struct BorderColor(pub Color);
#[derive(Bundle)]
pub struct Panel {
    responsive_grid_view: ResponsiveGridView,
    pub panel_type: PanelType,
    pub layer: Layer,
    pub content_area: PanelContentArea,
    pub panel_color: PanelColor,
    pub border_color: BorderColor,
    pub(crate) cache: Cache,
    pub(crate) difference: Difference,
    pub(crate) visibility: EnableVisibility,
    pub(crate) section: Section<InterfaceContext>,
}
impl Panel {
    pub const PADDING: (f32, f32) = (5.0, 5.0);
    pub const CORNER_DEPTH: f32 = 5f32;
    pub const LINE_WIDTH: f32 = 1f32;
    pub fn new<C: Into<Color>, L: Into<Layer>>(
        responsive_grid_view: ResponsiveGridView,
        panel_type: PanelType,
        layer: L,
        panel_color: C,
        border_color: C,
    ) -> Self {
        Self {
            responsive_grid_view,
            panel_type,
            layer: layer.into(),
            border_color: BorderColor(border_color.into()),
            content_area: PanelContentArea(Area::default()),
            panel_color: PanelColor(panel_color.into()),
            visibility: EnableVisibility::new(),
            cache: Cache::new(),
            difference: Difference::new(),
            section: Section::default(),
        }
    }
}
#[derive(Component)]
pub(crate) struct Cache {
    pub(crate) panel_type: Option<PanelType>,
    pub(crate) position: Option<Position<InterfaceContext>>,
    pub(crate) content_area: Option<Area<InterfaceContext>>,
    pub(crate) layer: Option<Layer>,
    pub(crate) panel_color: Option<Color>,
    pub(crate) border_color: Option<Color>,
}
impl Cache {
    pub(crate) fn new() -> Self {
        Self {
            panel_type: None,
            position: None,
            content_area: None,
            layer: None,
            panel_color: None,
            border_color: None,
        }
    }
}
#[derive(Component, Clone)]
pub(crate) struct Difference {
    pub(crate) panel_type: Option<PanelType>,
    pub(crate) position: Option<Position<InterfaceContext>>,
    pub(crate) content_area: Option<Area<InterfaceContext>>,
    pub(crate) layer: Option<Layer>,
    pub(crate) panel_color: Option<Color>,
    pub(crate) border_color: Option<Color>,
}
impl Difference {
    pub(crate) fn new() -> Self {
        Self {
            panel_type: None,
            position: None,
            content_area: None,
            layer: None,
            panel_color: None,
            border_color: None,
        }
    }
}
#[derive(Resource)]
pub(crate) struct Extraction {
    pub(crate) differences: HashMap<Entity, Difference>,
    pub(crate) removed: HashSet<Entity>,
}
impl Extraction {
    pub(crate) fn new() -> Self {
        Self {
            differences: HashMap::new(),
            removed: HashSet::new(),
        }
    }
}
