use bevy_ecs::bundle::Bundle;

use crate::coord::{CoordContext, Device, View};
use crate::coord::area::Area;
use crate::coord::position::Position;

#[derive(Bundle, Copy, Clone, Default, PartialEq, Debug)]
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
    pub fn from_ltrb(left: f32, top: f32, right: f32, bottom: f32) -> Self {
        Self {
            position: (left, top).into(),
            area: (right - left, bottom - top).into(),
        }
    }
    pub fn width(&self) -> f32 {
        return self.area.width;
    }
    pub fn height(&self) -> f32 {
        return self.area.height;
    }
    pub fn left(&self) -> f32 {
        return self.position.x;
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
        if position.x > self.left()
            && position.x < self.right()
            && position.y > self.top()
            && position.y < self.bottom()
        {
            return true;
        }
        return false;
    }
    pub fn intersection(&self, other: Self) -> Option<Self> {
        if !self.is_overlapping(other) {
            return None;
        }
        let top = self.top().max(other.top());
        let bottom = self.bottom().min(other.bottom());
        let left = self.left().max(other.left());
        let right = self.right().min(other.right());
        Option::from(Self::from_ltrb(left, top, right, bottom))
    }
}

impl Section<View> {
    pub(crate) fn to_scaled(&self, scale_factor: f64) -> Section<Device> {
        Section::<Device>::new(
            self.position.to_device(scale_factor),
            self.area.to_device(scale_factor),
        )
    }
}

impl<Context: CoordContext, P: Into<Position<Context>>, A: Into<Area<Context>>>
From<(Context, P, A)> for Section<Context>
{
    fn from(value: (Context, P, A)) -> Self {
        Self::new(value.1.into(), value.2.into())
    }
}
