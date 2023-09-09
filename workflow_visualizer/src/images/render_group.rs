use bevy_ecs::prelude::{NonSend, NonSendMut, Res, ResMut};

use crate::images::interface::Extraction;
use crate::images::renderer::{ImageFade, ImageHandle, ImageRenderer};
use crate::{
    AlignedUniform, Area, GfxSurface, InterfaceContext, Layer, Position, ScaleFactor,
    TextureCoordinates, Uniform,
};

pub(crate) struct ImageRenderGroup {
    pub(crate) image_name: ImageHandle,
    pub(crate) render_group_bind_group: wgpu::BindGroup,
    pub(crate) fade_and_layer: AlignedUniform<f32>,
    pub(crate) texture_coordinates: Uniform<TextureCoordinates>,
    pub(crate) placement: AlignedUniform<f32>,
}
impl ImageRenderGroup {
    pub(crate) fn new(
        name: ImageHandle,
        uniforms_bind_group: wgpu::BindGroup,
        fade_uniform: AlignedUniform<f32>,
        texture_coordinates: Uniform<TextureCoordinates>,
        placement: AlignedUniform<f32>,
    ) -> Self {
        Self {
            image_name: name,
            render_group_bind_group: uniforms_bind_group,
            fade_and_layer: fade_uniform,
            texture_coordinates,
            placement,
        }
    }
}
pub(crate) fn read_extraction(
    mut extraction: ResMut<Extraction>,
    #[cfg(not(target_family = "wasm"))] mut image_renderer: ResMut<ImageRenderer>,
    #[cfg(target_family = "wasm")] mut image_renderer: NonSendMut<ImageRenderer>,
    #[cfg(not(target_family = "wasm"))] gfx: Res<GfxSurface>,
    #[cfg(target_family = "wasm")] gfx: NonSend<GfxSurface>,
    scale_factor: Res<ScaleFactor>,
) {
    for entity in extraction.queued_remove.drain() {
        image_renderer.render_groups.remove(&entity);
    }
    for (entity, diff) in extraction.differences.drain() {
        if image_renderer.render_groups.get(&entity).is_none() {
            let fade_and_layer = AlignedUniform::new(
                &gfx.device,
                Some([
                    diff.fade.expect("fade").0,
                    diff.layer.expect("layer").z,
                    0.0,
                    0.0,
                ]),
            );
            let coordinates_uniform = Uniform::new(
                &gfx.device,
                image_renderer
                    .images
                    .get(&diff.name.clone().expect("name"))
                    .expect("images")
                    .coordinates,
            );
            let position = diff.pos.expect("pos");
            let area = diff.area.expect("area");
            let placement_uniform = AlignedUniform::new(
                &gfx.device,
                Some([position.x, position.y, area.width, area.height]),
            );
            let bind_group = gfx.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("images-uniform-bind-group"),
                layout: &image_renderer.render_group_uniforms_layout,
                entries: &[
                    fade_and_layer.uniform.bind_group_entry(0),
                    coordinates_uniform.bind_group_entry(1),
                    placement_uniform.uniform.bind_group_entry(2),
                ],
            });
            let render_group = ImageRenderGroup::new(
                diff.name.clone().expect("name"),
                bind_group,
                fade_and_layer,
                coordinates_uniform,
                placement_uniform,
            );
            image_renderer.render_groups.insert(entity, render_group);
        }
        if let Some(mut render_group) = image_renderer.render_groups.remove(&entity) {
            let mut placement_changed = false;
            if let Some(pos) = diff.pos {
                let pos = pos.to_device(scale_factor.factor());
                render_group.placement.set_aspect(0, pos.x);
                render_group.placement.set_aspect(1, pos.y);
                placement_changed = true;
            }
            if let Some(area) = diff.area {
                let area = area.to_device(scale_factor.factor());
                render_group.placement.set_aspect(2, area.width);
                render_group.placement.set_aspect(3, area.height);
                placement_changed = true;
            }
            if placement_changed {
                render_group.placement.update(&gfx.queue);
            }
            let mut fade_layer_changed = false;
            if let Some(fade) = diff.fade {
                render_group.fade_and_layer.set_aspect(0, fade.0);
                fade_layer_changed = true;
            }
            if let Some(layer) = diff.layer {
                render_group.fade_and_layer.set_aspect(1, layer.z);
                fade_layer_changed = true;
            }
            if fade_layer_changed {
                render_group.fade_and_layer.update(&gfx.queue);
            }
            if let Some(name) = diff.name {
                render_group.texture_coordinates.update(
                    &gfx.queue,
                    image_renderer
                        .images
                        .get(&name)
                        .expect("images")
                        .coordinates,
                );
                render_group.image_name = name;
            }
            image_renderer.render_groups.insert(entity, render_group);
        }
    }
}
