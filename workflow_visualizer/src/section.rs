use crate::area::Area;
use crate::coord::CoordContext;
use crate::position::Position;
use crate::{DeviceContext, InterfaceContext};
use bevy_ecs::bundle::Bundle;

#[derive(Bundle, Copy, Clone, PartialOrd, PartialEq, Default)]
pub struct Section<Context: CoordContext> {
    pub position: Position<Context>,
    pub area: Area<Context>,
}

impl<Context: CoordContext> Section<Context> {
    pub fn new<P: Into<Position<Context>>, A: Into<Area<Context>>>(position: P, area: A) -> Self {
        Self {
            position: position.into(),
            area: area.into(),
        }
    }
    pub fn from_left_top_right_bottom(left: f32, top: f32, right: f32, bottom: f32) -> Self {
        Self {
            position: (left, top).into(),
            area: (right - left, bottom - top).into(),
        }
    }
    pub fn width(&self) -> f32 {
        self.area.width
    }
    pub fn height(&self) -> f32 {
        self.area.height
    }
    pub fn left(&self) -> f32 {
        self.position.x
    }
    pub fn right(&self) -> f32 {
        self.position.x + self.area.width
    }
    pub fn top(&self) -> f32 {
        self.position.y
    }
    pub fn bottom(&self) -> f32 {
        self.position.y + self.area.height
    }
    pub fn is_touching(&self, other: Self) -> bool {
        self.left() <= other.right()
            && self.right() >= other.left()
            && self.top() <= other.bottom()
            && self.bottom() >= other.top()
    }
    pub fn is_overlapping(&self, other: Self) -> bool {
        self.left() < other.right()
            && self.right() > other.left()
            && self.top() < other.bottom()
            && self.bottom() > other.top()
    }
    pub fn contains(&self, position: Position<Context>) -> bool {
        if position.x >= self.left()
            && position.x <= self.right()
            && position.y >= self.top()
            && position.y <= self.bottom()
        {
            return true;
        }
        false
    }
    pub fn intersection(&self, other: Self) -> Option<Self> {
        if !self.is_overlapping(other) {
            return None;
        }
        let top = self.top().max(other.top());
        let bottom = self.bottom().min(other.bottom());
        let left = self.left().max(other.left());
        let right = self.right().min(other.right());
        Option::from(Self::from_left_top_right_bottom(left, top, right, bottom))
    }
}
impl Section<InterfaceContext> {
    pub(crate) fn to_device(&self, scale_factor: f64) -> Section<DeviceContext> {
        Section::<DeviceContext>::new(
            self.position.to_device(scale_factor),
            self.area.to_device(scale_factor),
        )
    }
}
impl Section<DeviceContext> {
    pub(crate) fn to_device(&self, scale_factor: f64) -> Section<InterfaceContext> {
        Section::<InterfaceContext>::new(
            self.position.to_ui(scale_factor),
            self.area.to_ui(scale_factor),
        )
    }
}
impl<Context: CoordContext, P: Into<Position<Context>>, A: Into<Area<Context>>> From<(P, A)>
    for Section<Context>
{
    fn from(value: (P, A)) -> Self {
        Self::new(value.0.into(), value.1.into())
    }
}
