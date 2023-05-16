use bevy_ecs::prelude::{EventReader, Events, IntoSystemConfig, Res, ResMut};

use crate::coord::area::Area;
use crate::coord::DeviceContext;
use crate::gfx::{GfxSurface, GfxSurfaceConfiguration, MsaaRenderAttachment};
use crate::sync::SyncPoint;
use crate::{Attach, Visualizer};

#[derive(Clone, Copy)]
pub struct WindowResize {
    pub size: Area<DeviceContext>,
    pub scale_factor: f64,
}

impl WindowResize {
    pub(crate) fn new(size: Area<DeviceContext>, scale_factor: f64) -> Self {
        Self { size, scale_factor }
    }
}
pub(crate) fn gfx_resize(
    gfx_surface: Res<GfxSurface>,
    mut gfx_surface_configuration: ResMut<GfxSurfaceConfiguration>,
    mut resize_events: EventReader<WindowResize>,
    mut msaa_attachment: ResMut<MsaaRenderAttachment>,
) {
    for resize in resize_events.iter() {
        gfx_surface_configuration.configuration.width =
            (resize.size.width as u32).min(gfx_surface.options.limits.max_texture_dimension_2d);
        gfx_surface_configuration.configuration.height =
            (resize.size.height as u32).min(gfx_surface.options.limits.max_texture_dimension_2d);
        *msaa_attachment = MsaaRenderAttachment::new(
            &gfx_surface,
            &gfx_surface_configuration,
            msaa_attachment.max,
            msaa_attachment.requested,
        );
        gfx_surface.surface.configure(
            &gfx_surface.device,
            &gfx_surface_configuration.configuration,
        );
    }
}
pub struct WindowAttachment;
impl Attach for WindowAttachment {
    fn attach(engen: &mut Visualizer) {
        engen
            .job
            .container
            .insert_resource(Events::<WindowResize>::default());
        engen.job.task(Visualizer::TASK_RENDER_MAIN).add_systems((
            gfx_resize.in_set(SyncPoint::Initialization),
            Events::<WindowResize>::update_system.in_set(SyncPoint::Event),
        ));
        engen
            .job
            .task(Visualizer::TASK_MAIN)
            .add_system(Events::<WindowResize>::update_system.in_set(SyncPoint::Event));
    }
}
