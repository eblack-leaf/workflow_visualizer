use bevy_ecs::prelude::{Bundle, Component};
#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Component, Copy, Clone, Default)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}
impl Position {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
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
#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Component, Copy, Clone, Default)]
pub struct Area {
    pub width: f32,
    pub height: f32,
}
impl Area {
    pub const fn new(width: f32, height: f32) -> Self {
        Self { width, height }
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
impl From<(u32, u32)> for Area {
    fn from(wh: (u32, u32)) -> Self {
        Self {
            width: wh.0 as f32,
            height: wh.1 as f32,
        }
    }
}
#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Component, Copy, Clone, Default)]
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
#[derive(Bundle, Clone, Default)]
pub struct Section {
    pub position: Position,
    pub area: Area,
}
impl Section {
    pub fn new(position: Position, area: Area) -> Self {
        Self { position, area }
    }
    pub fn width(&self) -> f32 {
        return self.area.width;
    }
    pub fn height(&self) -> f32 {
        return self.area.height;
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
