use bevy_ecs::prelude::Resource;
use crate::{Attachable, Engen};

impl Attachable for CanvasOptions {
    fn attach(self, engen: &mut Engen) {
        engen.render.container.insert_resource(self);
    }
}
#[derive(Clone, Resource)]
pub struct CanvasOptions {
    pub backends: wgpu::Backends,
    pub power_preferences: wgpu::PowerPreference,
    pub force_fallback_adapter: bool,
    pub features: wgpu::Features,
    pub limits: wgpu::Limits,
    pub present_mode: wgpu::PresentMode,
}
impl Default for CanvasOptions {
    fn default() -> Self {
        Self {
            backends: wgpu::Backends::PRIMARY,
            power_preferences: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            features: wgpu::Features::default(),
            limits: wgpu::Limits::default(),
            present_mode: wgpu::PresentMode::Fifo,
        }
    }
}
impl CanvasOptions {
    pub fn web_align(mut self) -> Self {
        self.backends = wgpu::Backends::all();
        self.limits = wgpu::Limits::downlevel_webgl2_defaults();
        return self;
    }
}
