use bevy_ecs::prelude::{Commands, Res};

pub struct DepthTexture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub format: wgpu::TextureFormat,
}
impl DepthTexture {
    pub fn new(
        device: &wgpu::Device,
        surface_configuration: &wgpu::SurfaceConfiguration,
        format: wgpu::TextureFormat,
    ) -> Self {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("depth texture"),
            size: wgpu::Extent3d {
                width: surface_configuration.width,
                height: surface_configuration.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        });
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        return Self {
            texture,
            view,
            format,
        };
    }
}
pub fn setup(
    device: Res<wgpu::Device>,
    surface_configuration: Res<wgpu::SurfaceConfiguration>,
    mut cmd: Commands,
) {
    let depth_texture = DepthTexture::new(
        &device,
        &surface_configuration,
        wgpu::TextureFormat::Depth32Float,
    );
    cmd.insert_resource(depth_texture);
}
