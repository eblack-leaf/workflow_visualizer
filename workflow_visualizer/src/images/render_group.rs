use bevy_ecs::prelude::{NonSend, NonSendMut, Res, ResMut};

use crate::images::interface::Extraction;
use crate::images::renderer::{ImageFade, ImageHandle, ImageRenderer};
use crate::{Area, GfxSurface, InterfaceContext, Layer, Position, TextureCoordinates, Uniform};

pub(crate) struct ImageRenderGroup {
    pub(crate) image_name: ImageHandle,
    pub(crate) render_group_bind_group: wgpu::BindGroup,
    pub(crate) fade_and_layer: Uniform<ImageFadeAndLayer>,
    pub(crate) cpu_fade_and_layer: ImageFadeAndLayer,
    pub(crate) texture_coordinates: Uniform<TextureCoordinates>,
    pub(crate) cpu_placement: ImagePlacement,
    pub(crate) placement: Uniform<ImagePlacement>,
}
#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Copy, Clone)]
pub(crate) struct ImagePlacement {
    placement: [f32; 4],
}
impl ImagePlacement {
    pub(crate) fn new(pos: Position<InterfaceContext>, area: Area<InterfaceContext>) -> Self {
        Self {
            placement: [pos.x, pos.y, area.width, area.height],
        }
    }
}
#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Copy, Clone)]
pub(crate) struct ImageFadeAndLayer {
    pub(crate) data: [f32; 4],
}
impl ImageFadeAndLayer {
    pub(crate) fn new(fade: ImageFade, layer: Layer) -> Self {
        Self {
            data: [fade.0, layer.z, 0.0, 0.0],
        }
    }
}
impl ImageRenderGroup {
    pub(crate) fn new(
        name: ImageHandle,
        uniforms_bind_group: wgpu::BindGroup,
        fade_uniform: Uniform<ImageFadeAndLayer>,
        texture_coordinates: Uniform<TextureCoordinates>,
        placement: Uniform<ImagePlacement>,
        image_fade_and_layer: ImageFadeAndLayer,
        image_placement: ImagePlacement,
    ) -> Self {
        Self {
            image_name: name,
            render_group_bind_group: uniforms_bind_group,
            fade_and_layer: fade_uniform,
            cpu_fade_and_layer: image_fade_and_layer,
            texture_coordinates,
            cpu_placement: image_placement,
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
) {
    for entity in extraction.queued_remove.drain() {
        image_renderer.render_groups.remove(&entity);
    }
    for (entity, diff) in extraction.differences.drain() {
        if image_renderer.render_groups.get(&entity).is_none() {
            let image_fade_and_layer =
                ImageFadeAndLayer::new(diff.fade.expect("fade"), diff.layer.expect("layer"));
            let fade_and_layer = Uniform::new(&gfx.device, image_fade_and_layer);
            let coordinates_uniform = Uniform::new(
                &gfx.device,
                image_renderer
                    .images
                    .get(&diff.name.clone().expect("name"))
                    .expect("images")
                    .coordinates,
            );
            let image_placement =
                ImagePlacement::new(diff.pos.expect("pos"), diff.area.expect("area"));
            let placement_uniform = Uniform::new(&gfx.device, image_placement);
            let bind_group = gfx.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("images-uniform-bind-group"),
                layout: &image_renderer.render_group_uniforms_layout,
                entries: &[
                    fade_and_layer.bind_group_entry(0),
                    coordinates_uniform.bind_group_entry(1),
                    placement_uniform.bind_group_entry(2),
                ],
            });
            let render_group = ImageRenderGroup::new(
                diff.name.clone().expect("name"),
                bind_group,
                fade_and_layer,
                coordinates_uniform,
                placement_uniform,
                image_fade_and_layer,
                image_placement,
            );
            image_renderer.render_groups.insert(entity, render_group);
        }
        if let Some(mut render_group) = image_renderer.render_groups.remove(&entity) {
            let mut placement_changed = false;
            if let Some(pos) = diff.pos {
                render_group.cpu_placement.placement[0] = pos.x;
                render_group.cpu_placement.placement[1] = pos.y;
                placement_changed = true;
            }
            if let Some(area) = diff.area {
                render_group.cpu_placement.placement[2] = area.width;
                render_group.cpu_placement.placement[3] = area.height;
                placement_changed = true;
            }
            if placement_changed {
                render_group
                    .placement
                    .update(&gfx.queue, render_group.cpu_placement);
            }
            let mut fade_layer_changed = false;
            if let Some(fade) = diff.fade {
                render_group.cpu_fade_and_layer.data[0] = fade.0;
                fade_layer_changed = true;
            }
            if let Some(layer) = diff.layer {
                render_group.cpu_fade_and_layer.data[1] = layer.z;
                fade_layer_changed = true;
            }
            if fade_layer_changed {
                render_group
                    .fade_and_layer
                    .update(&gfx.queue, render_group.cpu_fade_and_layer);
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
