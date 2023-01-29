use crate::coord::area::Area;
use crate::coord::position::Position;
use bevy_ecs::bundle::Bundle;

#[derive(Bundle, Copy, Clone, Default, PartialEq)]
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
        self.left() < other.right()
            && self.right() > other.left()
            && self.top() < other.bottom()
            && self.bottom() > other.top()
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
}

impl From<((f32, f32), (f32, f32))> for Section {
    fn from(data: ((f32, f32), (f32, f32))) -> Self {
        Self {
            position: data.0.into(),
            area: data.1.into(),
        }
    }
}
