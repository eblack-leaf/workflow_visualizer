use crate::area::Area;
use crate::coord::DeviceContext;
use crate::gfx::{GfxSurface, GfxSurfaceConfiguration, MsaaRenderAttachment};
use crate::{Attach, Engen};
use bevy_ecs::prelude::{EventReader, Events, Res, ResMut};

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
pub(crate) fn resize(
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
    fn attach(engen: &mut Engen) {
        engen
            .frontend
            .container
            .insert_resource(Events::<WindowResize>::default());
        engen
            .backend
            .container
            .insert_resource(Events::<WindowResize>::default());
        engen.backend.main.add_system(resize);
        engen
            .frontend
            .main
            .add_system(Events::<WindowResize>::update_system);
        engen
            .backend
            .main
            .add_system(Events::<WindowResize>::update_system);
    }
}
