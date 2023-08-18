use crate::{TextureBindGroup, Uniform};
use crate::image::renderer::ImageFade;

pub(crate) struct ImageRenderGroup {
    pub(crate) image_bind_group: TextureBindGroup,
    pub(crate) fade_bind_group: wgpu::BindGroup,
    pub(crate) fade_uniform: Uniform<ImageFade>,
}
