use bevy_ecs::change_detection::ResMut;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{Commands, Query, Res};

use crate::{ScaleFactor, Viewport};
use crate::gfx::{GfxSurface, GfxSurfaceConfiguration, MsaaRenderAttachment};
use crate::icon::{IconMeshAddRequest, IconRenderer};
use crate::icon::cache::Differences;

pub(crate) fn read_add_requests(
    mut renderer: ResMut<IconRenderer>,
    mut cmd: Commands,
    requests: Query<(Entity, &IconMeshAddRequest)>,
    gfx_surface: Res<GfxSurface>,
) {
    for (entity, request) in requests.iter() {
        cmd.entity(entity).despawn();
        renderer.add_mesh(
            &gfx_surface,
            request.icon_key,
            request.icon_mesh.to_gpu(&gfx_surface),
            request.max,
        )
    }
}

pub(crate) fn process_differences(
    mut renderer: ResMut<IconRenderer>,
    differences: Res<Differences>,
    scale_factor: Res<ScaleFactor>,
    gfx_surface: Res<GfxSurface>,
) {
    renderer.process_differences(&differences, scale_factor.factor, &gfx_surface);
}

pub(crate) fn setup(
    gfx_surface: Res<GfxSurface>,
    gfx_surface_config: Res<GfxSurfaceConfiguration>,
    viewport: Res<Viewport>,
    msaa_attachment: Res<MsaaRenderAttachment>,
    mut cmd: Commands,
) {
    cmd.insert_resource(IconRenderer::new(
        &gfx_surface,
        &gfx_surface_config,
        &viewport,
        msaa_attachment.requested,
    ));
}
