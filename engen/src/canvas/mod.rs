use bevy_ecs::prelude::Resource;
use wgpu::{CompositeAlphaMode, SurfaceError, SurfaceTexture};
use winit::window::Window;

pub use crate::canvas::options::CanvasOptions;
pub use crate::canvas::viewport::Viewport;
use crate::Position;
pub(crate) use crate::visibility::Visibility;
pub(crate) use crate::visibility::visibility;

mod options;
mod viewport;

#[cfg_attr(not(target_arch = "wasm32"), derive(Resource))]
pub struct CanvasWindow(pub Window);

#[derive(Resource)]
pub struct Canvas {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface_configuration: wgpu::SurfaceConfiguration,
    pub viewport: Viewport,
    pub options: CanvasOptions,
}

impl Canvas {
    pub async fn new(window: &Window, options: CanvasOptions) -> Self {
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
        let viewport = Viewport::new(
            &device,
            (surface_configuration.width, surface_configuration.height).into(),
        );
        Self {
            surface,
            device,
            queue,
            surface_configuration,
            viewport,
            options,
        }
    }
    pub(crate) fn adjust(&mut self, width: u32, height: u32) {
        self.surface_configuration.width = width;
        self.surface_configuration.height = height;
        self.viewport
            .adjust_area(&self.device, &self.queue, width, height);
        self.surface
            .configure(&self.device, &self.surface_configuration);
    }
    pub(crate) fn update_viewport_offset(&mut self, offset: Position) {
        self.viewport.update_offset(&self.queue, offset);
    }
    pub(crate) fn surface_texture(&self) -> Option<SurfaceTexture> {
        let surface_texture = match self.surface.get_current_texture() {
            Ok(surface_texture) => Some(surface_texture),
            Err(err) => match err {
                SurfaceError::Timeout => None,
                SurfaceError::Outdated => {
                    self.surface
                        .configure(&self.device, &self.surface_configuration);
                    Some(
                        self.surface
                            .get_current_texture()
                            .expect("configuring did not solve surface outdated"),
                    )
                }
                SurfaceError::Lost => {
                    self.surface
                        .configure(&self.device, &self.surface_configuration);
                    Some(
                        self.surface
                            .get_current_texture()
                            .expect("configuring did not solve surface lost"),
                    )
                }
                SurfaceError::OutOfMemory => {
                    panic!("gpu out of memory");
                }
            },
        };
        surface_texture
    }
}
