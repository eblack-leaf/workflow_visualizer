use std::fmt::{Debug, Formatter};

use bevy_ecs::prelude::Resource;
use tracing::{info, instrument, trace};
use wgpu::{ColorTargetState, PrimitiveState, TextureView};
use winit::window::Window;

/// Options for instantiating the Gfx module
#[derive(Clone)]
pub struct GfxOptions {
    pub backends: wgpu::Backends,
    pub power_preferences: wgpu::PowerPreference,
    pub force_fallback_adapter: bool,
    pub features: wgpu::Features,
    pub limits: wgpu::Limits,
    pub present_mode: wgpu::PresentMode,
    pub msaa: u32,
}
impl Debug for GfxOptions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "GfxOptions")
    }
}
impl GfxOptions {
    pub fn downlevel_align(mut self) -> Self {
        self.backends = wgpu::Backends::all();
        self.limits = wgpu::Limits::downlevel_webgl2_defaults();
        self.msaa = 1;
        self
    }
    pub fn limited_environment() -> Self {
        Self::native_defaults().downlevel_align()
    }
    pub fn native_defaults() -> Self {
        Self {
            backends: wgpu::Backends::PRIMARY,
            power_preferences: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            features: wgpu::Features::default(),
            limits: wgpu::Limits::default(),
            present_mode: wgpu::PresentMode::Fifo,
            msaa: 1,
        }
    }
    pub fn with_msaa(mut self, msaa: u32) -> Self {
        self.msaa = msaa;
        self
    }
}
/// Core structs for interop with wgpu
#[derive(Resource)]
pub struct GfxSurface {
    pub instance: wgpu::Instance,
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub options: GfxOptions,
}
pub(crate) type GfxStack = (GfxSurface, GfxSurfaceConfiguration, MsaaRenderAdapter);
impl GfxSurface {
    #[instrument]
    pub(crate) async fn new(window: &Window, options: GfxOptions) -> GfxStack {
        trace!(
            "obtaining gfx stack. Backends: {:?}, Features: {:?}",
            options.backends,
            options.features
        );
        let instance_descriptor = wgpu::InstanceDescriptor {
            backends: options.backends,
            ..wgpu::InstanceDescriptor::default()
        };
        let instance = wgpu::Instance::new(instance_descriptor);
        trace!("creating surface");
        let surface = unsafe {
            instance
                .create_surface(window)
                .expect("could not create surface")
        };
        trace!(
            "requesting adapter with {:?} fallback_adapter: {:?}",
            options.power_preferences,
            options.force_fallback_adapter
        );
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: options.power_preferences,
                force_fallback_adapter: options.force_fallback_adapter,
                compatible_surface: Some(&surface),
            })
            .await
            .expect("adapter request failed");
        info!("requesting device/queue");
        let features = options.features | wgpu::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES;
        let mut limits = options.limits.clone().using_resolution(adapter.limits());
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("device/queue"),
                    features,
                    limits,
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
        trace!("surface format: {:?}", surface_format);
        let surface_configuration = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: window
                .inner_size()
                .width
                .min(options.limits.max_texture_dimension_2d),
            height: window
                .inner_size()
                .height
                .min(options.limits.max_texture_dimension_2d),
            present_mode: options.present_mode,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![surface_format],
        };
        surface.configure(&device, &surface_configuration);
        let gfx_surface = Self {
            instance,
            surface,
            device,
            queue,
            options: options.clone(),
        };
        let gfx_surface_config = GfxSurfaceConfiguration::new(surface_configuration);
        let msaa_render_attachment =
            Self::configure_msaa(options, adapter, &gfx_surface, &gfx_surface_config);
        (gfx_surface, gfx_surface_config, msaa_render_attachment)
    }

    fn configure_msaa(
        options: GfxOptions,
        adapter: wgpu::Adapter,
        gfx_surface: &GfxSurface,
        gfx_surface_config: &GfxSurfaceConfiguration,
    ) -> MsaaRenderAdapter {
        let msaa_flags = adapter
            .get_texture_format_features(gfx_surface_config.configuration.format)
            .flags;
        let max_sample_count = {
            if msaa_flags.contains(wgpu::TextureFormatFeatureFlags::MULTISAMPLE_X8) {
                8
            } else if msaa_flags.contains(wgpu::TextureFormatFeatureFlags::MULTISAMPLE_X4) {
                4
            } else if msaa_flags.contains(wgpu::TextureFormatFeatureFlags::MULTISAMPLE_X2) {
                2
            } else {
                1
            }
        };

        MsaaRenderAdapter::new(
            gfx_surface,
            gfx_surface_config,
            max_sample_count,
            options.msaa,
        )
    }
    pub fn filled_triangle_list(&self) -> PrimitiveState {
        wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            unclipped_depth: false,
            polygon_mode: wgpu::PolygonMode::Fill,
            conservative: false,
        }
    }
    pub(crate) fn surface_texture(
        &self,
        surface_configuration: &GfxSurfaceConfiguration,
    ) -> Option<wgpu::SurfaceTexture> {
        match self.surface.get_current_texture() {
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
        }
    }
}

#[derive(Resource)]
pub struct GfxSurfaceConfiguration {
    pub(crate) configuration: wgpu::SurfaceConfiguration,
}

impl GfxSurfaceConfiguration {
    pub(crate) fn new(configuration: wgpu::SurfaceConfiguration) -> Self {
        Self { configuration }
    }
    pub fn alpha_color_target_state(&self) -> ColorTargetState {
        wgpu::ColorTargetState {
            format: self.configuration.format,
            blend: Some(wgpu::BlendState::ALPHA_BLENDING),
            write_mask: Default::default(),
        }
    }
}
/// Used for setting the multi-sampling/anti-aliasing
#[derive(Resource)]
pub struct MsaaRenderAdapter {
    max: u32,
    requested: u32,
    view: Option<wgpu::TextureView>,
}

impl MsaaRenderAdapter {
    pub fn max(&self) -> u32 {
        self.max
    }
    pub fn requested(&self) -> u32 {
        self.requested
    }
    pub fn view(&self) -> Option<&TextureView> {
        self.view.as_ref()
    }
    pub fn multisample_state(&self) -> wgpu::MultisampleState {
        wgpu::MultisampleState {
            count: self.requested(),
            ..wgpu::MultisampleState::default()
        }
    }
    pub(crate) fn new(
        gfx_surface: &GfxSurface,
        gfx_surface_config: &GfxSurfaceConfiguration,
        max: u32,
        requested: u32,
    ) -> Self {
        let requested = requested.min(max);
        match requested > 1u32 {
            true => {
                let texture_extent = wgpu::Extent3d {
                    width: gfx_surface_config.configuration.width,
                    height: gfx_surface_config.configuration.height,
                    depth_or_array_layers: 1,
                };
                let descriptor = wgpu::TextureDescriptor {
                    size: texture_extent,
                    mip_level_count: 1,
                    sample_count: requested,
                    dimension: wgpu::TextureDimension::D2,
                    format: gfx_surface_config.configuration.format,
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                    label: Some("msaa render attachment"),
                    view_formats: &[],
                };
                let texture = gfx_surface.device.create_texture(&descriptor);
                let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
                Self {
                    view: Some(view),
                    max,
                    requested,
                }
            }
            false => Self {
                view: None,
                max: 1,
                requested,
            },
        }
    }
}
