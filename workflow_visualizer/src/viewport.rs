use bevy_ecs::change_detection::{Res, ResMut};
use bevy_ecs::event::EventReader;
use bevy_ecs::prelude::{Commands, IntoSystemConfig, Resource};
use nalgebra::matrix;

use crate::coord::area::Area;
use crate::coord::layer::Layer;
use crate::coord::position::Position;
use crate::coord::section::Section;
use crate::coord::DeviceContext;
use crate::gfx::{GfxSurface, GfxSurfaceConfiguration, MsaaRenderAttachment};
use crate::uniform::Uniform;
use crate::window::{gfx_resize, WindowResize};
use crate::{Attach, Engen, Extract, InterfaceContext, Job, ScaleFactor, SyncPoint};

#[derive(Resource)]
pub struct Viewport {
    pub(crate) cpu: CpuViewport,
    pub(crate) gpu: GpuViewport,
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub(crate) uniform: Uniform<GpuViewport>,
    pub(crate) depth_texture: wgpu::Texture,
    pub(crate) depth_format: wgpu::TextureFormat,
    pub(crate) offset: ViewportOffset,
    pub(crate) offset_uniform: Uniform<ViewportOffset>,
}

#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Copy, Clone, Default, PartialEq)]
pub(crate) struct ViewportOffset {
    pub(crate) offset: [f32; 4],
}

impl ViewportOffset {
    pub(crate) fn new(position: Position<DeviceContext>) -> Self {
        Self {
            offset: [position.x, position.y, 0.0, 0.0],
        }
    }
}

impl Viewport {
    pub(crate) fn new(device: &wgpu::Device, area: Area<DeviceContext>, sample_count: u32) -> Self {
        let depth = 100u32.into();
        let cpu_viewport = CpuViewport::new(area, depth);
        let gpu_viewport = cpu_viewport.gpu_viewport();
        let uniform = Uniform::new(device, gpu_viewport);
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("view bind group layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });
        let offset = ViewportOffset::new(Position::<DeviceContext>::new(0.0, 0.0));
        let offset_uniform = Uniform::new(device, offset);
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("view bind group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform.buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: offset_uniform.buffer.as_entire_binding(),
                },
            ],
        });
        let depth_format = wgpu::TextureFormat::Depth24PlusStencil8;
        let depth_texture = depth_texture(device, area, depth_format, sample_count);
        Self {
            cpu: cpu_viewport,
            gpu: gpu_viewport,
            bind_group,
            bind_group_layout,
            uniform,
            depth_texture,
            depth_format,
            offset,
            offset_uniform,
        }
    }
    pub(crate) fn as_section(&self) -> Section<DeviceContext> {
        Section::new(
            Position::<DeviceContext>::new(self.offset.offset[0], self.offset.offset[1]),
            self.cpu.area,
        )
    }
    pub(crate) fn adjust_area(
        &mut self,
        gfx_surface: &GfxSurface,
        width: u32,
        height: u32,
        sample_count: u32,
    ) {
        let area = Area::<DeviceContext>::new(width as f32, height as f32);
        self.cpu = CpuViewport::new(area, 100u32.into());
        self.gpu = self.cpu.gpu_viewport();
        self.uniform.update(&gfx_surface.queue, self.gpu);
        self.depth_texture =
            depth_texture(&gfx_surface.device, area, self.depth_format, sample_count);
    }
    pub(crate) fn update_offset(&mut self, queue: &wgpu::Queue, offset: Position<DeviceContext>) {
        self.offset = ViewportOffset::new(offset);
        self.offset_uniform.update(queue, self.offset);
    }
}

fn depth_texture(
    device: &wgpu::Device,
    area: Area<DeviceContext>,
    format: wgpu::TextureFormat,
    sample_count: u32,
) -> wgpu::Texture {
    device.create_texture(&wgpu::TextureDescriptor {
        label: Some("depth texture"),
        size: wgpu::Extent3d {
            width: area.width as u32,
            height: area.height as u32,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[format],
    })
}

#[derive(Resource)]
pub(crate) struct CpuViewport {
    pub(crate) area: Area<DeviceContext>,
    pub(crate) layer: Layer,
    pub(crate) orthographic: nalgebra::Matrix4<f32>,
}

impl CpuViewport {
    pub(crate) fn new(area: Area<DeviceContext>, layer: Layer) -> Self {
        Self {
            area,
            layer,
            orthographic: matrix![2f32/area.width, 0.0, 0.0, -1.0;
                                    0.0, 2f32/-area.height, 0.0, 1.0;
                                    0.0, 0.0, 1.0/layer.z, 0.0;
                                    0.0, 0.0, 0.0, 1.0],
        }
    }
    pub(crate) fn gpu_viewport(&self) -> GpuViewport {
        self.orthographic.data.0.into()
    }
    pub(crate) fn far_layer(&self) -> f32 {
        self.layer.z
    }
}

#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Copy, Clone, Resource)]
pub struct GpuViewport {
    pub matrix: [[f32; 4]; 4],
}

impl From<[[f32; 4]; 4]> for GpuViewport {
    fn from(matrix: [[f32; 4]; 4]) -> Self {
        Self { matrix }
    }
}

pub(crate) fn viewport_attach(
    gfx_surface: Res<GfxSurface>,
    gfx_surface_configuration: Res<GfxSurfaceConfiguration>,
    msaa_attachment: Res<MsaaRenderAttachment>,
    mut cmd: Commands,
) {
    let area = Area::<DeviceContext>::new(
        gfx_surface_configuration.configuration.width as f32,
        gfx_surface_configuration.configuration.height as f32,
    );
    cmd.insert_resource(Viewport::new(
        &gfx_surface.device,
        area,
        msaa_attachment.requested,
    ));
}

pub(crate) fn viewport_resize(
    gfx_surface: Res<GfxSurface>,
    gfx_surface_configuration: Res<GfxSurfaceConfiguration>,
    mut viewport: ResMut<Viewport>,
    mut resize_events: EventReader<WindowResize>,
    msaa_attachment: Res<MsaaRenderAttachment>,
) {
    for _resize in resize_events.iter() {
        viewport.adjust_area(
            &gfx_surface,
            gfx_surface_configuration.configuration.width,
            gfx_surface_configuration.configuration.height,
            msaa_attachment.requested,
        );
    }
}
pub(crate) fn frontend_area_adjust(
    mut resize_events: EventReader<WindowResize>,
    mut viewport_handle: ResMut<ViewportHandle>,
    scale_factor: Res<ScaleFactor>,
) {
    for event in resize_events.iter() {
        viewport_handle.section.area = event.size.to_ui(scale_factor.factor);
    }
}
#[derive(Resource)]
pub struct ViewportHandle {
    pub section: Section<InterfaceContext>,
    position_dirty: bool,
}

impl ViewportHandle {
    pub(crate) fn new(section: Section<InterfaceContext>) -> Self {
        Self {
            section,
            position_dirty: false,
        }
    }
    pub fn position_adjust(&mut self, adjust: Position<InterfaceContext>) {
        self.section.position += adjust;
        self.position_dirty = true;
    }
}

#[derive(Resource)]
pub struct ViewportHandleAdjust {
    pub adjust: Option<Position<InterfaceContext>>,
}

impl ViewportHandleAdjust {
    pub(crate) fn new() -> Self {
        Self { adjust: None }
    }
}

pub(crate) fn adjust_position(
    mut visible_bounds: ResMut<ViewportHandle>,
    mut visible_bounds_position_adjust: ResMut<ViewportHandleAdjust>,
) {
    if let Some(adjust) = visible_bounds_position_adjust.adjust.take() {
        visible_bounds.position_adjust(adjust);
    }
}

#[derive(Resource)]
pub(crate) struct ViewportOffsetUpdate {
    pub(crate) update: Option<Position<DeviceContext>>,
}

impl ViewportOffsetUpdate {
    pub(crate) fn new() -> Self {
        Self { update: None }
    }
}

pub(crate) fn viewport_read_offset(
    mut viewport_offset_update: ResMut<ViewportOffsetUpdate>,
    mut viewport: ResMut<Viewport>,
    gfx_surface: Res<GfxSurface>,
) {
    if let Some(update) = viewport_offset_update.update.take() {
        viewport.update_offset(&gfx_surface.queue, update);
    }
}

impl Extract for ViewportHandle {
    fn extract(frontend: &mut Job, backend: &mut Job) {
        let scale_factor = frontend
            .container
            .get_resource::<ScaleFactor>()
            .expect("no scale factor")
            .factor;
        let mut viewport_handle = frontend
            .container
            .get_resource_mut::<ViewportHandle>()
            .expect("no viewport handle");
        if viewport_handle.position_dirty {
            backend
                .container
                .get_resource_mut::<ViewportOffsetUpdate>()
                .expect("no viewport offset update")
                .update
                .replace(viewport_handle.section.position.to_device(scale_factor));
            viewport_handle.position_dirty = false;
        }
    }
}
pub struct ViewportAttachment;

impl Attach for ViewportAttachment {
    fn attach(engen: &mut Engen) {
        engen.add_extraction::<ViewportHandle>();
        engen.frontend.main.add_systems((
            adjust_position.in_set(SyncPoint::Initialization),
            frontend_area_adjust.in_set(SyncPoint::Initialization),
        ));
        engen
            .backend
            .startup
            .add_system(viewport_attach.in_set(SyncPoint::Initialization));
        engen.backend.main.add_systems((
            viewport_read_offset,
            viewport_resize
                .in_set(SyncPoint::Initialization)
                .after(gfx_resize),
        ));
        engen
            .backend
            .container
            .insert_resource(ViewportOffsetUpdate::new());
        let gfx_surface_configuration = engen
            .backend
            .container
            .get_resource::<GfxSurfaceConfiguration>()
            .expect("no gfx surface config");
        let scale_factor = engen
            .frontend
            .container
            .get_resource::<ScaleFactor>()
            .expect("no scale factor")
            .factor;
        let surface_area: Area<DeviceContext> = (
            gfx_surface_configuration.configuration.width,
            gfx_surface_configuration.configuration.height,
        )
            .into();
        let visible_section = ((0u32, 0u32), surface_area.to_ui(scale_factor)).into();
        engen
            .frontend
            .container
            .insert_resource(ViewportHandle::new(visible_section));
        engen
            .frontend
            .container
            .insert_resource(ViewportHandleAdjust::new());
    }
}
