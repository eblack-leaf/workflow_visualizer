use std::ops::{Add, AddAssign};

use bevy_ecs::prelude::{Bundle, Component, Resource};

#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Component, Copy, Clone, Default, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new<F: Into<f32>>(x: F, y: F) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }
    pub fn apply(&mut self, movement: Movement) {
        self.x += movement.x;
        self.y += movement.y;
    }
}

impl From<(f32, f32)> for Position {
    fn from(xy: (f32, f32)) -> Self {
        Self { x: xy.0, y: xy.1 }
    }
}

impl From<(u32, u32)> for Position {
    fn from(xy: (u32, u32)) -> Self {
        Self {
            x: xy.0 as f32,
            y: xy.1 as f32,
        }
    }
}

impl From<(usize, usize)> for Position {
    fn from(xy: (usize, usize)) -> Self {
        Self {
            x: xy.0 as f32,
            y: xy.1 as f32,
        }
    }
}

impl Add for Position {
    type Output = Position;
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.x + rhs.x, self.y + rhs.y)
    }
}

#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Component, Copy, Clone, Default, PartialEq)]
pub struct Movement {
    pub x: f32,
    pub y: f32,
}

impl Movement {
    pub fn new<F: Into<f32>>(x: F, y: F) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }
}

#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Component, Copy, Clone, Default, PartialEq)]
pub struct Area {
    pub width: f32,
    pub height: f32,
}

impl Area {
    pub const fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
    pub fn apply(&mut self, scale: Scale) {
        self.width += scale.width;
        self.height += scale.height;
    }
}

impl From<(f32, f32)> for Area {
    fn from(wh: (f32, f32)) -> Self {
        Self {
            width: wh.0,
            height: wh.1,
        }
    }
}

#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Component, Copy, Clone, Default, PartialEq)]
pub struct Scale {
    pub width: f32,
    pub height: f32,
}

impl Scale {
    pub fn new<F: Into<f32>>(w: F, h: F) -> Self {
        Self {
            width: w.into(),
            height: h.into(),
        }
    }
}

impl From<(u32, u32)> for Area {
    fn from(wh: (u32, u32)) -> Self {
        Self {
            width: wh.0 as f32,
            height: wh.1 as f32,
        }
    }
}

impl From<(usize, usize)> for Area {
    fn from(wh: (usize, usize)) -> Self {
        Self {
            width: wh.0 as f32,
            height: wh.1 as f32,
        }
    }
}

#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Component, Copy, Clone, Default, PartialEq)]
pub struct Depth {
    pub layer: f32,
}

impl Depth {
    pub const fn new(layer: f32) -> Self {
        Self { layer }
    }
}

impl From<f32> for Depth {
    fn from(layer: f32) -> Self {
        Self { layer }
    }
}

impl From<u32> for Depth {
    fn from(layer: u32) -> Self {
        Self {
            layer: layer as f32,
        }
    }
}

#[derive(Bundle, Copy, Clone, Default)]
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
}

impl From<((f32, f32), (f32, f32))> for Section {
    fn from(data: ((f32, f32), (f32, f32))) -> Self {
        Self {
            position: data.0.into(),
            area: data.1.into(),
        }
    }
}

impl From<(Position, Area)> for Section {
    fn from(data: (Position, Area)) -> Self {
        Self {
            position: data.0,
            area: data.1,
        }
    }
}

#[derive(Bundle, Clone, Default)]
pub struct Panel {
    #[bundle]
    pub section: Section,
    pub depth: Depth,
}

impl Panel {
    pub fn new(section: Section, depth: Depth) -> Self {
        Self { section, depth }
    }
    pub fn width(&self) -> f32 {
        return self.section.area.width;
    }
    pub fn height(&self) -> f32 {
        return self.section.area.height;
    }
    pub fn layer(&self) -> f32 {
        return self.depth.layer;
    }
}

impl From<((f32, f32), (f32, f32), f32)> for Panel {
    fn from(data: ((f32, f32), (f32, f32), f32)) -> Self {
        Self {
            section: (data.0, data.1).into(),
            depth: data.2.into(),
        }
    }
}
