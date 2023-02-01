use bevy_ecs::prelude::{EventReader, Res, ResMut, Resource, SystemStage};
use winit::window::Window;

use crate::{Preparation, BackendStages, Stove};
use crate::window::Resize;

#[derive(Clone)]
pub struct GfxOptions {
    pub backends: wgpu::Backends,
    pub power_preferences: wgpu::PowerPreference,
    pub force_fallback_adapter: bool,
    pub features: wgpu::Features,
    pub limits: wgpu::Limits,
    pub present_mode: wgpu::PresentMode,
}

impl GfxOptions {
    pub fn web_align(mut self) -> Self {
        self.backends = wgpu::Backends::all();
        self.limits = wgpu::Limits::downlevel_webgl2_defaults();
        return self;
    }
    pub fn web() -> Self {
        Self::native().web_align()
    }
    pub fn native() -> Self {
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

#[derive(Resource)]
pub(crate) struct GfxSurface {
    pub(crate) surface: wgpu::Surface,
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
}

impl GfxSurface {
    pub(crate) async fn new(
        window: &Window,
        options: GfxOptions,
    ) -> (Self, GfxSurfaceConfiguration) {
        let instance_descriptor = wgpu::InstanceDescriptor {
            backends: options.backends,
            ..wgpu::InstanceDescriptor::default()
        };
        let instance = wgpu::Instance::new(instance_descriptor);
        let surface = unsafe {
            instance
                .create_surface(window)
                .expect("could not create surface")
        };
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
                None,
            )
            .await
            .expect("device/queue request failed");
        let surface_format = *surface
            .get_capabilities(&adapter)
            .formats
            .first()
            .expect("surface format unsupported");
        let surface_configuration = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: window.inner_size().width,
            height: window.inner_size().height,
            present_mode: options.present_mode,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![surface_format],
        };
        surface.configure(&device, &surface_configuration);
        (
            Self {
                surface,
                device,
                queue,
            },
            GfxSurfaceConfiguration::new(surface_configuration),
        )
    }
    pub(crate) fn surface_texture(
        &self,
        surface_configuration: &GfxSurfaceConfiguration,
    ) -> Option<wgpu::SurfaceTexture> {
        let surface_texture = match self.surface.get_current_texture() {
            Ok(surface_texture) => Some(surface_texture),
            Err(err) => match err {
                wgpu::SurfaceError::Timeout => None,
                wgpu::SurfaceError::Outdated => {
                    self.surface
                        .configure(&self.device, &surface_configuration.configuration);
                    Some(
                        self.surface
                            .get_current_texture()
                            .expect("configuring did not solve surface outdated"),
                    )
                }
                wgpu::SurfaceError::Lost => {
                    self.surface
                        .configure(&self.device, &surface_configuration.configuration);
                    Some(
                        self.surface
                            .get_current_texture()
                            .expect("configuring did not solve surface lost"),
                    )
                }
                wgpu::SurfaceError::OutOfMemory => {
                    panic!("gpu out of memory");
                }
            },
        };
        surface_texture
    }
}

#[derive(Resource)]
pub(crate) struct GfxSurfaceConfiguration {
    pub(crate) configuration: wgpu::SurfaceConfiguration,
}

impl GfxSurfaceConfiguration {
    pub(crate) fn new(configuration: wgpu::SurfaceConfiguration) -> Self {
        Self { configuration }
    }
}

pub(crate) fn resize(
    gfx_surface: Res<GfxSurface>,
    mut gfx_surface_configuration: ResMut<GfxSurfaceConfiguration>,
    mut resize_events: EventReader<Resize>,
) {
    for resize in resize_events.iter() {
        gfx_surface_configuration.configuration.width = resize.size.width as u32;
        gfx_surface_configuration.configuration.height = resize.size.height as u32;
        gfx_surface.surface.configure(
            &gfx_surface.device,
            &gfx_surface_configuration.configuration,
        );
    }
}
