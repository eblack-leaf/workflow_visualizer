use bevy_ecs::prelude::{Event, EventReader, Events, IntoSystemConfigs, Res, ResMut};
use tracing::trace;

use crate::coord::area::Area;

use crate::gfx::{GfxSurface, GfxSurfaceConfiguration, MsaaRenderAdapter};
use crate::sync::SyncPoint;
use crate::visualizer::{Attach, Visualizer};
use crate::{WindowAppearanceContext, WindowAppearanceFactor};

/// Event for triggering Window Resizing behaviour
#[derive(Event, Clone, Copy)]
pub struct WindowResize {
    pub size: Area<WindowAppearanceContext>,
    pub scale_factor: f32,
}

impl WindowResize {
    pub(crate) fn new(size: Area<WindowAppearanceContext>, scale_factor: f32) -> Self {
        Self { size, scale_factor }
    }
}
pub(crate) fn gfx_resize(
    #[cfg(not(target_family = "wasm"))] gfx_surface: Res<GfxSurface>,
    #[cfg(target_family = "wasm")] gfx_surface: NonSend<GfxSurface>,
    #[cfg(not(target_family = "wasm"))] mut gfx_surface_configuration: ResMut<
        GfxSurfaceConfiguration,
    >,
    #[cfg(target_family = "wasm")] mut gfx_surface_configuration: NonSendMut<
        GfxSurfaceConfiguration,
    >,
    mut resize_events: EventReader<WindowResize>,
    #[cfg(not(target_family = "wasm"))] mut msaa_attachment: ResMut<MsaaRenderAdapter>,
    #[cfg(target_family = "wasm")] mut msaa_attachment: NonSendMut<MsaaRenderAdapter>,
    mut window_appearance_factor: ResMut<WindowAppearanceFactor>,
) {
    for resize in resize_events.iter() {
        trace!("resizing event: {:?}", resize.size);
        let requested = Area::new(resize.size.width, resize.size.height);
        let actual = Area::new(
            requested
                .width
                .min(gfx_surface.options.limits.max_texture_dimension_2d as f32),
            requested
                .height
                .min(gfx_surface.options.limits.max_texture_dimension_2d as f32),
        );
        *window_appearance_factor = WindowAppearanceFactor::new(requested, actual);
        gfx_surface_configuration.configuration.width = actual.width as u32;
        gfx_surface_configuration.configuration.height = actual.height as u32;
        *msaa_attachment = MsaaRenderAdapter::new(
            &gfx_surface,
            &gfx_surface_configuration,
            msaa_attachment.max(),
            msaa_attachment.requested(),
        );
        gfx_surface.surface.configure(
            &gfx_surface.device,
            &gfx_surface_configuration.configuration,
        );
    }
}
pub(crate) struct WindowAttachment;
impl Attach for WindowAttachment {
    fn attach(engen: &mut Visualizer) {
        engen
            .job
            .container
            .insert_resource(Events::<WindowResize>::default());
        engen
            .job
            .task(Visualizer::TASK_RENDER_MAIN)
            .add_systems((gfx_resize.in_set(SyncPoint::Initialization),));
        engen
            .job
            .task(Visualizer::TASK_STARTUP)
            .add_systems((Events::<WindowResize>::update_system.in_set(SyncPoint::Event),));
    }
}
