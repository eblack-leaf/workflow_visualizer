use bevy_ecs::prelude::{Res, ResMut};
use crate::image::renderer::{ImageFade, ImageName, ImageRenderer};
use crate::{Area, GfxSurface, InterfaceContext, Layer, Position, TextureCoordinates, Uniform};
use crate::image::interface::Extraction;

pub(crate) struct ImageRenderGroup {
    pub(crate) image_name: ImageName,
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
            placement: [pos.x, pos.y, area.width, area.height]
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
            data: [fade.0, layer.z, 0.0, 0.0]
        }
    }
}
impl ImageRenderGroup {
    pub(crate) fn new(
        name: ImageName,
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
pub(crate) fn read_extraction(mut extraction: ResMut<Extraction>, mut image_renderer: ResMut<ImageRenderer>, gfx: Res<GfxSurface>) {
    for entity in extraction.queued_remove.drain() {
        image_renderer.render_groups.remove(&entity);
    }
    for (entity, diff) in extraction.differences.drain() {
        if image_renderer.render_groups.get(&entity).is_none() {
            let image_fade_and_layer = ImageFadeAndLayer::new(diff.fade.expect("fade"), diff.layer.expect("layer"));
            let fade_uniform = Uniform::new(&gfx.device, image_fade_and_layer);
            let coordinates_uniform = Uniform::new(&gfx.device, image_renderer.images.get(&diff.name.expect("name")).expect("image").coordinates);
            let image_placement = ImagePlacement::new(diff.pos.expect("pos"), diff.area.expect("area"));
            let placement_uniform = Uniform::new(&gfx.device, image_placement);
            let bind_group = gfx.device.create_bind_group(&wgpu::BindGroupDescriptor{
                label: Some("image-uniform-bind-group"),
                layout: &image_renderer.render_group_uniforms_layout,
                entries: &[
                    fade_uniform.bind_group_entry(0),
                    coordinates_uniform.bind_group_entry(1),
                    placement_uniform.bind_group_entry(2),
                ],
            });
            let render_group = ImageRenderGroup::new(diff.name.expect("name"), bind_group, fade_uniform, coordinates_uniform, placement_uniform, image_fade_and_layer, image_placement);
            image_renderer.render_groups.insert(entity, render_group);
        }
        if let Some(pos) = diff.pos {
            if let Some(render_group) = image_renderer.render_groups.get_mut(&entity) {
                render_group.cpu_placement.placement[0] = pos.x;
                render_group.cpu_placement.placement[1] = pos.y;
            }
        }
    }
}
