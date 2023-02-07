use bevy_ecs::bundle::Bundle;

use crate::coord::{ScaledArea, ScaledPosition};
use crate::coord::area::Area;
use crate::coord::position::Position;

#[derive(Bundle, Copy, Clone, Default, PartialEq, Debug)]
pub struct Section {
    pub position: Position,
    pub area: Area,
}

impl Section {
    pub fn new<P: Into<Position>, A: Into<Area>>(position: P, area: A) -> Self {
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
    pub(crate) fn to_scaled(&self, scale_factor: f64) -> ScaledSection {
        ScaledSection::new(
            self.position.to_scaled(scale_factor),
            self.area.to_scaled(scale_factor),
        )
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
    pub fn is_overlapping(&self, other: Section) -> bool {
        self.left() <= other.right()
            && self.right() >= other.left()
            && self.top() <= other.bottom()
            && self.bottom() >= other.top()
    }
    pub fn contains(&self, position: Position) -> bool {
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

impl<P: Into<Position>, A: Into<Area>> From<(P, A)> for Section {
    fn from(value: (P, A)) -> Self {
        Self::new(value.0.into(), value.1.into())
    }
}

#[derive(Bundle, Copy, Clone, Default, PartialEq, Debug)]
pub struct ScaledSection {
    pub position: ScaledPosition,
    pub area: ScaledArea,
}

impl ScaledSection {
    pub fn new<P: Into<ScaledPosition>, A: Into<ScaledArea>>(position: P, area: A) -> Self {
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
    pub fn is_overlapping(&self, other: ScaledSection) -> bool {
        self.left() <= other.right()
            && self.right() >= other.left()
            && self.top() <= other.bottom()
            && self.bottom() >= other.top()
    }
    pub fn contains(&self, position: ScaledPosition) -> bool {
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

impl<SP: Into<ScaledPosition>, SA: Into<ScaledArea>> From<(SP, SA)> for ScaledSection {
    fn from(value: (SP, SA)) -> Self {
        Self::new(value.0.into(), value.1.into())
    }
}
