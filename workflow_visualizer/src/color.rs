use bevy_ecs::prelude::Component;

/// RGBA colors
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
    pub const DARK_ORANGE: (f32, f32, f32) = (0.035, 0.0125, 0.00);
    pub const DARK_CYAN: (f32, f32, f32) = (0.0, 0.025, 0.0125);
    pub const CYAN: (f32, f32, f32) = (0.0, 0.75, 0.35);
    pub const CYAN_MEDIUM: (f32, f32, f32) = (0.0, 0.45, 0.145);
    pub const OFF_WHITE: (f32, f32, f32) = (0.8, 0.8, 0.8);
    pub const DARK_GREY: (f32, f32, f32) = (0.01, 0.01, 0.01);
    pub const MEDIUM_GREY: (f32, f32, f32) = (0.05, 0.05, 0.05);
    pub const GREY: (f32, f32, f32) = (0.25, 0.25, 0.25);
    pub const BLACK: (f32, f32, f32) = (0.0, 0.0, 0.0);
    pub const LIGHT_RED: (f32, f32, f32) = (0.9, 0.02, 0.02);
    pub const RED: (f32, f32, f32) = (0.8, 0.01, 0.01);
    pub const MEDIUM_RED: (f32, f32, f32) = (0.4, 0.005, 0.005);
    pub const DARK_RED: (f32, f32, f32) = (0.2, 0.0025, 0.0025);
    pub const LIGHT_RED_ORANGE: (f32, f32, f32) = (0.9, 0.35, 0.1);
    pub const RED_ORANGE: (f32, f32, f32) = (0.7, 0.225, 0.045);
    pub const MEDIUM_RED_ORANGE: (f32, f32, f32) = (0.4, 0.125, 0.025);
    pub const DARK_RED_ORANGE: (f32, f32, f32) = (0.2, 0.07, 0.0125);
    pub const LIGHT_GREEN: (f32, f32, f32) = (0.02, 0.9, 0.02);
    pub const GREEN: (f32, f32, f32) = (0.01, 0.7, 0.01);
    pub const MEDIUM_GREEN: (f32, f32, f32) = (0.001, 0.16, 0.001);
    pub const DARK_GREEN: (f32, f32, f32) = (0.0005, 0.08, 0.0005);
    pub const BLUE_DARK: (f32, f32, f32) = (0.0, 0.05, 0.10);
    pub const BLUE: (f32, f32, f32) = (0.1, 0.1, 0.9);
    pub const OFF_BLACK: (f32, f32, f32) = (0.005, 0.005, 0.005);
    pub const BLANK: (f32, f32, f32, f32) = (0.0, 0.0, 0.0, 0.0);
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
    pub fn with_alpha(mut self, alpha: f32) -> Self {
        self.alpha = alpha.min(1.0);
        self
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
