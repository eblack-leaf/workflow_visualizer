use crate::{TextureCoordinates, Uniform};
use crate::image::renderer::{ImageFade, ImageName};

pub(crate) struct ImageRenderGroup {
    pub(crate) image_name: ImageName,
    pub(crate) render_group_bind_group: wgpu::BindGroup,
    pub(crate) fade_uniform: Uniform<ImageFade>,
    pub(crate) texture_coordinates: Uniform<TextureCoordinates>,
}

impl ImageRenderGroup {
    pub(crate) fn new(name: ImageName, render_group_bind_group: wgpu::BindGroup, fade_uniform: Uniform<ImageFade>, texture_coordinates: Uniform<TextureCoordinates>) -> Self {
        Self {
            image_name: name,
            render_group_bind_group,
            fade_uniform,
            texture_coordinates,
        }
    }
}
