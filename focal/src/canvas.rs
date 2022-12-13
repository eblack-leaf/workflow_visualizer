use crate::{Gfx, GfxOptions};
use wgpu::CompositeAlphaMode;
use winit::window::Window;

pub struct Canvas {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface_configuration: wgpu::SurfaceConfiguration,
}

impl Canvas {
    pub async fn new(window: &Window, options: GfxOptions) -> Self {
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
        Self {
            surface,
            device,
            queue,
            surface_configuration,
        }
    }
}
pub(crate) fn adjust(gfx: &mut Gfx, width: u32, height: u32) {
    let mut canvas = gfx.canvas.as_mut().unwrap();
    canvas.surface_configuration.width = width;
    canvas.surface_configuration.height = height;
    canvas
        .surface
        .configure(&canvas.device, &canvas.surface_configuration);
    gfx.viewport
        .as_mut()
        .unwrap()
        .adjust(&canvas.device, &canvas.queue, width, height);
}
