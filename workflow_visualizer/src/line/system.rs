use bevy_ecs::prelude::{Changed, Entity, Or, Query, RemovedComponents, Res, ResMut};

use crate::{Area, Color, GfxSurface, InterfaceContext, Layer, Position, ScaleFactor, Visibility};
use crate::line::line_render::LineRenderPoints;
use crate::line::LineRender;
use crate::line::renderer::{LayerAndHooks, LineRenderer, LineRenderGpu, LineRenderGroup};
use crate::path::Path;

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
    for (path, mut pos, mut area) in lines.iter_mut() {
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
    for (entity, path, mut line_render, mut line_render_points) in paths.iter_mut() {
        let mut scaled = Vec::new();
        for point in path.points.iter() {
            scaled.push(point.to_device(scale_factor.factor()));
        }
        *line_render = LineRender::new(scaled.len() - 1);
        *line_render_points = LineRenderPoints { points: scaled };
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
    gfx: Res<GfxSurface>,
    mut removed: RemovedComponents<LineRender>,
    mut line_renderer: ResMut<LineRenderer>,
) {
    for (entity, line_render, layer, color, line_render_points, visibility) in paths.iter() {
        if visibility.visible() {
            let render_group = LineRenderGroup::new(
                LineRenderGpu::new(&gfx, &line_render_points.points),
                line_render.capacity,
                LayerAndHooks::new(layer.z, 0f32, 0f32, 0f32),
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
    mut line_renderer: ResMut<LineRenderer>,
) {
    for (entity, layer) in lines.iter() {
        if let Some(group) = line_renderer.render_groups.get_mut(&entity) {
            group.layer_and_hooks.aspects[0] = layer.z;
            group.layer_and_hooks_dirty = true;
        }
    }
}

pub(crate) fn push_color(
    lines: Query<(Entity, &Color), Changed<Color>>,
    mut line_renderer: ResMut<LineRenderer>,
) {
    for (entity, color) in lines.iter() {
        if let Some(group) = line_renderer.render_groups.get_mut(&entity) {
            group.color = *color;
            group.color_dirty = true;
        }
    }
}

pub(crate) fn push_uniforms(mut line_renderer: ResMut<LineRenderer>, gfx: Res<GfxSurface>) {
    for group in line_renderer.render_groups.values_mut() {
        if group.color_dirty {
            group.color_uniform.update(&gfx.queue, group.color);
            group.color_dirty = false;
        }
        if group.layer_and_hooks_dirty {
            group
                .layer_and_hooks_uniform
                .update(&gfx.queue, group.layer_and_hooks);
            group.layer_and_hooks_dirty = false;
        }
    }
}
