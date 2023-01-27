use bytemuck::{Pod, Zeroable};

#[derive(Clone, Copy)]
pub struct Area {
    pub width: f32,
    pub height: f32,
}

impl Area {
    pub fn new<F: Into<f32>>(width: F, height: F) -> Self {
        Self {
            width: width.into(),
            height: height.into(),
        }
    }
}

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
    pub(crate) fn to_gpu(&self, scale_factor: f64) -> GpuPosition {
        todo!()
    }
}

#[derive(Pod, Zeroable, Copy, Clone)]
pub(crate) struct GpuPosition {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

pub struct Depth {
    pub layer: f32,
}

impl Depth {
    pub fn new<T: Into<u32>>(layer: T) -> Self {
        let layer = layer.into();
        Self {
            layer: layer as f32,
        }
    }
}

impl From<u32> for Depth {
    fn from(value: u32) -> Self {
        Depth::new(value)
    }
}
