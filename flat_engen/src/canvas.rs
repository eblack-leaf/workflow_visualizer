use bevy_ecs::change_detection::Mut;
use bevy_ecs::prelude::Resource;
use winit::window::Window;

use crate::Engen;

#[derive(Clone)]
pub struct CanvasOptions {
    pub backends: wgpu::Backends,
    pub power_preferences: wgpu::PowerPreference,
    pub force_fallback_adapter: bool,
    pub features: wgpu::Features,
    pub limits: wgpu::Limits,
    pub present_mode: wgpu::PresentMode,
}

impl CanvasOptions {
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

impl Default for CanvasOptions {
    fn default() -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        {
            CanvasOptions::native()
        }
        #[cfg(target_arch = "wasm32")]
        {
            CanvasOptions::web()
        }
    }
}

#[derive(Resource)]
pub(crate) struct Canvas {
    pub(crate) surface: wgpu::Surface,
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
    pub(crate) surface_configuration: wgpu::SurfaceConfiguration,
}

impl Canvas {
    pub(crate) async fn new(window: &Window, options: CanvasOptions) -> Self {
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
        Self {
            surface,
            device,
            queue,
            surface_configuration,
        }
    }
    pub(crate) fn surface_texture(&self) -> Option<wgpu::SurfaceTexture> {
        let surface_texture = match self.surface.get_current_texture() {
            Ok(surface_texture) => Some(surface_texture),
            Err(err) => match err {
                wgpu::SurfaceError::Timeout => None,
                wgpu::SurfaceError::Outdated => {
                    self.surface
                        .configure(&self.device, &self.surface_configuration);
                    Some(
                        self.surface
                            .get_current_texture()
                            .expect("configuring did not solve surface outdated"),
                    )
                }
                wgpu::SurfaceError::Lost => {
                    self.surface
                        .configure(&self.device, &self.surface_configuration);
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
    pub(crate) fn adjust(&mut self, width: u32, height: u32) {
        self.surface_configuration.width = width;
        self.surface_configuration.height = height;
        self.surface
            .configure(&self.device, &self.surface_configuration);
    }
    pub(crate) async fn attach(window: &Window, engen: &mut Engen) {
        let options = engen.engen_options.canvas_options.clone();
        let canvas = Canvas::new(window, options).await;
        engen.backend.container.insert_resource(canvas);
    }
    pub(crate) fn get(engen: &Engen) -> &Canvas {
        engen
            .backend
            .container
            .get_resource::<Canvas>()
            .expect("no canvas attached")
    }
    pub(crate) fn get_mut(engen: &mut Engen) -> Mut<'_, Canvas> {
        engen
            .backend
            .container
            .get_resource_mut::<Canvas>()
            .expect("no canvas attached")
    }
}
