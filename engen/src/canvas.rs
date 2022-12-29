use bevy_ecs::prelude::Resource;
use wgpu::CompositeAlphaMode;
use winit::window::Window;
use crate::{Launcher, LaunchOptions};
use crate::viewport::Viewport;
#[derive(Resource)]
pub struct Canvas {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface_configuration: wgpu::SurfaceConfiguration,
    pub viewport: Viewport,
}
impl Canvas {
    pub async fn new(window: &Window, options: LaunchOptions) -> Self {
        let instance = wgpu::Instance::new(options.backends);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: options.power_preferences,
                force_fallback_adapter: options.force_fallback_adapter,
                compatible_surface: Some(&surface),
            })
            .await
            .expect("adapter request failed");
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("device/queue"),
                    features: options.features,
                    limits: options.limits.clone(),
                },
                // Some(Path::new("/home/omi-voshuli/note-ifications/wgpu_traces")),
                None,
            )
            .await
            .expect("device/queue request failed");
        let surface_configuration = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: *surface
                .get_supported_formats(&adapter)
                .first()
                .expect("surface format unsupported"),
            width: window.inner_size().width,
            height: window.inner_size().height,
            present_mode: options.present_mode,
            alpha_mode: CompositeAlphaMode::Auto,
        };
        surface.configure(&device, &surface_configuration);
        let viewport = Viewport::new(&device, (surface_configuration.width, surface_configuration.height).into());
        Self {
            surface,
            device,
            queue,
            surface_configuration,
            viewport
        }
    }
    pub(crate) fn adjust(&mut self, width: u32, height: u32) {
        self.surface_configuration.width = width;
        self.surface_configuration.height = height;
        self.viewport.adjust(&self.device, &self.queue, width, height);
    }
}
pub(crate) fn adjust(launcher: &mut Launcher, width: u32, height: u32) {
    let mut canvas = launcher.render.job.container.get_resource_mut::<Canvas>().expect("no canvas attached");
    canvas.adjust(width, height);
}