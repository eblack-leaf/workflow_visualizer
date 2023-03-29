use std::collections::{HashMap, HashSet};

use bevy_ecs::prelude::{Bundle, Component, Entity, Resource};

pub use attachment::PanelAttachment;
pub use system::calc_content_area;

use crate::view::{ViewArea, ViewPosition};
use crate::{
    Area, Color, Coordinate, EnableVisibility, InterfaceContext, Layer, Location, Position, Section,
};

mod attachment;
mod renderer;
mod system;
mod vertex;

#[derive(Component, Copy, Clone)]
pub struct PanelContentArea(pub Area<InterfaceContext>);
#[derive(Bundle)]
pub struct Panel {
    pub view_position: ViewPosition,
    pub view_area: ViewArea,
    pub layer: Layer,
    pub content_area: PanelContentArea,
    pub color: Color,
    pub(crate) cache: Cache,
    pub(crate) difference: Difference,
    pub(crate) visibility: EnableVisibility,
    pub(crate) section: Section<InterfaceContext>,
}
impl Panel {
    pub const PADDING: (f32, f32) = (5.0, 5.0);
    pub const CORNER_DEPTH: f32 = 5f32;
    pub fn new<C: Into<Color>>(
        view_position: ViewPosition,
        view_area: ViewArea,
        layer: Layer,
        color: C,
    ) -> Self {
        Self {
            view_position,
            view_area,
            layer,
            content_area: PanelContentArea(Area::default()),
            color: color.into(),
            visibility: EnableVisibility::new(),
            cache: Cache::new(),
            difference: Difference::new(),
            section: Section::default(),
        }
    }
}
#[derive(Component)]
pub(crate) struct Cache {
    pub(crate) position: Option<Position<InterfaceContext>>,
    pub(crate) content_area: Option<Area<InterfaceContext>>,
    pub(crate) layer: Option<Layer>,
    pub(crate) color: Option<Color>,
}
impl Cache {
    pub(crate) fn new() -> Self {
        Self {
            position: None,
            content_area: None,
            layer: None,
            color: None,
        }
    }
}
#[derive(Component, Clone)]
pub(crate) struct Difference {
    pub(crate) position: Option<Position<InterfaceContext>>,
    pub(crate) content_area: Option<Area<InterfaceContext>>,
    pub(crate) layer: Option<Layer>,
    pub(crate) color: Option<Color>,
}
impl Difference {
    pub(crate) fn new() -> Self {
        Self {
            position: None,
            content_area: None,
            layer: None,
            color: None,
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
