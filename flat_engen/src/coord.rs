use bytemuck::{Pod, Zeroable};

#[derive(Clone, Copy)]
pub struct Area {
    pub width: f32,
    pub height: f32,
}

impl Area {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
    pub(crate) fn to_gpu(&self, scale_factor: f64) -> GpuArea {
        todo!()
    }
}

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy)]
pub(crate) struct GpuArea {
    pub(crate) width: f32,
    pub(crate) height: f32,
}

impl GpuArea {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

#[derive(Copy, Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub(crate) fn to_gpu(&self, scale_factor: f64) -> GpuPosition {
        todo!()
    }
}

#[repr(C)]
#[derive(Pod, Zeroable, Copy, Clone)]
pub(crate) struct GpuPosition {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

impl GpuPosition {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[repr(C)]
#[derive(Pod, Zeroable, Copy, Clone)]
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
