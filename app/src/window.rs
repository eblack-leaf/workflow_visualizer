use crate::coord::Area;
use crate::depth_texture::DepthTexture;
use crate::viewport;
use crate::Signal;
use bevy_ecs::prelude::{Res, ResMut};

pub struct Resize {
    pub area: Area,
}
impl Resize {
    pub fn new<T: Into<Area>>(area: T) -> Self {
        Self { area: area.into() }
    }
}
pub fn resize(
    mut resize_signal: ResMut<Signal<Resize>>,
    surface: Res<wgpu::Surface>,
    device: Res<wgpu::Device>,
    queue: Res<wgpu::Queue>,
    mut config: ResMut<wgpu::SurfaceConfiguration>,
    mut depth_texture: ResMut<DepthTexture>,
    mut viewport: ResMut<viewport::Viewport>,
    mut viewport_binding: ResMut<viewport::Binding>,
) {
    if let Some(resize) = &resize_signal.receive() {
        config.width = resize.area.width as u32;
        config.height = resize.area.height as u32;
        surface.configure(&device, &config);
        *viewport = viewport::Viewport::new(
            (resize.area.width, resize.area.height).into(),
            viewport.depth,
        );
        viewport_binding
            .uniform
            .update(&queue, viewport.gpu_viewport());
        *depth_texture = DepthTexture::new(&device, &config, depth_texture.format);
    }
}
