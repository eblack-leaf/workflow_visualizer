use crate::line::line_render::LineRenderPoints;
use crate::line::renderer::{LineRenderGpu, LineRenderGroup, LineRenderer};
use crate::line::LineRender;
use crate::path::Path;
use crate::{
    AlignedUniform, Area, Color, GfxSurface, InterfaceContext, Layer, Position, ScaleFactor,
    Visibility,
};
#[cfg(not(target_family = "wasm"))]
use bevy_ecs::prelude::ResMut;
use bevy_ecs::prelude::{Changed, Entity, Or, Query, RemovedComponents, Res};
#[cfg(target_family = "wasm")]
use bevy_ecs::prelude::{NonSend, NonSendMut};

// before ResolveVisibility after Reconfigure.set_path
pub(crate) fn calc_section(
    mut lines: Query<
        (
            &Path,
            &mut Position<InterfaceContext>,
            &mut Area<InterfaceContext>,
        ),
        Changed<Path>,
    >,
) {
    for (_path, mut pos, mut area) in lines.iter_mut() {
        // TODO remove this and calc section of line by farthest bounds
        *pos = (100, 100).into();
        *area = (100, 100).into();
        // end TODO
    }
}

pub(crate) fn scale_path(
    scale_factor: Res<ScaleFactor>,
    mut paths: Query<(Entity, &Path, &mut LineRender, &mut LineRenderPoints), Changed<Path>>,
) {
    for (_entity, path, mut line_render, mut line_render_points) in paths.iter_mut() {
        let mut scaled = Vec::new();
        for point in path.points.iter() {
            scaled.push(point.to_device(scale_factor.factor()));
        }
        if !scaled.is_empty() {
            *line_render = LineRender::new(scaled.len().checked_sub(1).unwrap_or_default());
            *line_render_points = LineRenderPoints { points: scaled };
        }
    }
}

pub(crate) fn create_render_group(
    paths: Query<
        (
            Entity,
            &LineRender,
            &Layer,
            &Color,
            &LineRenderPoints,
            &Visibility,
        ),
        Or<(Changed<LineRenderPoints>, Changed<Visibility>)>,
    >,
    #[cfg(not(target_family = "wasm"))] gfx: Res<GfxSurface>,
    #[cfg(target_family = "wasm")] gfx: NonSend<GfxSurface>,
    mut removed: RemovedComponents<LineRender>,
    #[cfg(not(target_family = "wasm"))] mut line_renderer: ResMut<LineRenderer>,
    #[cfg(target_family = "wasm")] mut line_renderer: NonSendMut<LineRenderer>,
) {
    for (entity, line_render, layer, color, line_render_points, visibility) in paths.iter() {
        if visibility.visible() {
            let render_group = LineRenderGroup::new(
                LineRenderGpu::new(&gfx, &line_render_points.points),
                line_render.capacity,
                AlignedUniform::new(&gfx.device, Some([layer.z, 0.0, 0.0, 0.0])),
                *color,
                &gfx,
                &line_renderer.bind_group_layout,
            );
            line_renderer.render_groups.insert(entity, render_group);
        } else {
            line_renderer.render_groups.remove(&entity);
        }
    }
    for entity in removed.iter() {
        line_renderer.render_groups.remove(&entity);
    }
}

pub(crate) fn push_layer(
    lines: Query<(Entity, &Layer), Changed<Layer>>,
    #[cfg(not(target_family = "wasm"))] mut line_renderer: ResMut<LineRenderer>,
    #[cfg(target_family = "wasm")] mut line_renderer: NonSendMut<LineRenderer>,
) {
    for (entity, layer) in lines.iter() {
        if let Some(group) = line_renderer.render_groups.get_mut(&entity) {
            group.layer_and_hooks.set_aspect(0, layer.z);
            group.layer_and_hooks_dirty = true;
        }
    }
}

pub(crate) fn push_color(
    lines: Query<(Entity, &Color), Changed<Color>>,
    #[cfg(not(target_family = "wasm"))] mut line_renderer: ResMut<LineRenderer>,
    #[cfg(target_family = "wasm")] mut line_renderer: NonSendMut<LineRenderer>,
) {
    for (entity, color) in lines.iter() {
        if let Some(group) = line_renderer.render_groups.get_mut(&entity) {
            group.color = *color;
            group.color_dirty = true;
        }
    }
}

pub(crate) fn push_uniforms(
    #[cfg(not(target_family = "wasm"))] mut line_renderer: ResMut<LineRenderer>,
    #[cfg(target_family = "wasm")] mut line_renderer: NonSendMut<LineRenderer>,
    #[cfg(not(target_family = "wasm"))] gfx: Res<GfxSurface>,
    #[cfg(target_family = "wasm")] gfx: NonSend<GfxSurface>,
) {
    for group in line_renderer.render_groups.values_mut() {
        if group.color_dirty {
            group.color_uniform.update(&gfx.queue, group.color);
            group.color_dirty = false;
        }
        if group.layer_and_hooks_dirty {
            group.layer_and_hooks.update(&gfx.queue);
            group.layer_and_hooks_dirty = false;
        }
    }
}
