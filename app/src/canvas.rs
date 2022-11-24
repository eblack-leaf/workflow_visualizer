use std::path::Path;
use winit::window::Window;
#[derive(Clone)]
pub struct Options {
    pub backends: wgpu::Backends,
    pub power_preferences: wgpu::PowerPreference,
    pub force_fallback_adapter: bool,
    pub features: wgpu::Features,
    pub limits: wgpu::Limits,
    pub present_mode: wgpu::PresentMode,
}
impl Options {
    pub fn defaults() -> Self {
        Self {
            backends: wgpu::Backends::PRIMARY,
            power_preferences: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            features: wgpu::Features::default(),
            limits: wgpu::Limits::default(),
            present_mode: wgpu::PresentMode::Fifo,
        }
    }
    pub fn web_defaults() -> Self {
        let mut options = Self::defaults();
        options.backends = wgpu::Backends::all();
        options.limits = wgpu::Limits::downlevel_webgl2_defaults();
        return options;
    }
}
pub type Canvas = (
    wgpu::Surface,
    wgpu::Device,
    wgpu::Queue,
    wgpu::SurfaceConfiguration,
);
pub async fn canvas(window: &Window, options: Options) -> Canvas {
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
    };
    surface.configure(&device, &surface_configuration);
    return (surface, device, queue, surface_configuration);
}
