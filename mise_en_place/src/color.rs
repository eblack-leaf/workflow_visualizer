use bevy_ecs::prelude::Component;

#[repr(C)]
#[derive(Component, bytemuck::Pod, bytemuck::Zeroable, Copy, Clone, PartialEq)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

impl Default for Color {
    fn default() -> Self {
        Self::rgb(1.0, 1.0, 1.0)
    }
}

impl Color {
    pub const WHITE: (f32, f32, f32) = (1.0, 1.0, 1.0);
    pub const OFF_WHITE: (f32, f32, f32) = (0.8, 0.8, 0.8);
    pub const BLACK: (f32, f32, f32) = (0.0, 0.0, 0.0);
    pub const RED: (f32, f32, f32) = (0.9, 0.1, 0.1);
    pub const GREEN: (f32, f32, f32) = (0.1, 0.9, 0.1);
    pub const BLUE: (f32, f32, f32) = (0.1, 0.1, 0.9);
    pub const OFF_BLACK: (f32, f32, f32) = (0.2, 0.2, 0.2);
    pub fn rgb(red: f32, green: f32, blue: f32) -> Self {
        Self {
            red,
            green,
            blue,
            alpha: 1f32,
        }
    }
    pub fn rgba(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }
}

impl From<Color> for wgpu::Color {
    fn from(color: Color) -> Self {
        Self {
            r: color.red as f64,
            g: color.green as f64,
            b: color.blue as f64,
            a: color.alpha as f64,
        }
    }
}

impl From<(f32, f32, f32)> for Color {
    fn from(rgb: (f32, f32, f32)) -> Self {
        Self::rgb(rgb.0, rgb.1, rgb.2)
    }
}

impl From<(f32, f32, f32, f32)> for Color {
    fn from(rgba: (f32, f32, f32, f32)) -> Self {
        Self::rgba(rgba.0, rgba.1, rgba.2, rgba.3)
    }
}
