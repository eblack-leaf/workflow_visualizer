use bevy_ecs::prelude::Component;
#[repr(C)]
#[derive(Component, bytemuck::Pod, bytemuck::Zeroable, Copy, Clone)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}
impl Color {
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
    pub fn flatten(&self) -> [f32; 4] {
        return [self.red, self.green, self.blue, self.alpha];
    }
}
