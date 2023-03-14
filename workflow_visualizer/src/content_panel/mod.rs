mod renderer;
mod vertex;

use crate::{Area, Color, InterfaceContext, Location, Panel};
use bevy_ecs::prelude::{Bundle, Component};
#[derive(Component, Copy, Clone)]
pub struct LineWidth(pub u32);
#[derive(Component, Copy, Clone)]
pub struct LineColor(pub Color);
#[derive(Component, Copy, Clone)]
pub struct ContentArea(pub Area<InterfaceContext>);
#[derive(Bundle)]
pub struct ContentPanel {
    pub location: Location<InterfaceContext>,
    pub content_area: ContentArea,
    pub line_width: LineWidth,
    pub line_color: LineColor,
}
impl ContentPanel {
    pub fn new<
        L: Into<Location<InterfaceContext>>,
        A: Into<Area<InterfaceContext>>,
        C: Into<Color>,
    >(
        location: L,
        content_area: A,
        line_width: u32,
        line_color: C,
    ) -> Self {
        Self {
            location: location.into(),
            content_area: ContentArea(content_area.into()),
            line_width: LineWidth(line_width),
            line_color: LineColor(line_color.into()),
        }
    }
}
