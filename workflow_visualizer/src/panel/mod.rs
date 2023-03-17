use std::collections::{HashMap, HashSet};

use bevy_ecs::prelude::{Bundle, Component, Entity, Resource};

pub use attachment::PanelAttachment;
pub use system::calc_area_from_content_area;

use crate::{Area, Color, EnableVisibility, InterfaceContext, Layer, Location, Position};

mod attachment;
mod renderer;
mod system;
mod vertex;

#[derive(Component, Copy, Clone)]
pub struct LineWidth(pub u32);
#[derive(Component, Copy, Clone)]
pub struct LineColor(pub Color);
#[derive(Component, Copy, Clone)]
pub struct ContentArea(pub Area<InterfaceContext>);
#[derive(Component, Copy, Clone)]
pub struct Padding(pub Area<InterfaceContext>);
#[derive(Bundle)]
pub struct Panel {
    pub location: Location<InterfaceContext>,
    pub content_area: ContentArea,
    pub color: Color,
    pub(crate) padding: Padding,
    pub(crate) cache: Cache,
    pub(crate) difference: Difference,
    pub(crate) visibility: EnableVisibility,
    pub line_width: LineWidth,
    pub line_color: LineColor,
    pub(crate) area: Area<InterfaceContext>,
}
impl Panel {
    pub fn new<
        L: Into<Location<InterfaceContext>>,
        A: Into<Area<InterfaceContext>>,
        C: Into<Color>,
    >(
        location: L,
        color: C,
        padding: A,
        line_width: u32,
        line_color: C,
    ) -> Self {
        Self {
            location: location.into(),
            content_area: ContentArea(Area::default()),
            color: color.into(),
            padding: Padding(padding.into()),
            line_width: LineWidth(line_width),
            line_color: LineColor(line_color.into()),
            visibility: EnableVisibility::new(),
            cache: Cache::new(),
            difference: Difference::new(),
            area: Area::default(),
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
