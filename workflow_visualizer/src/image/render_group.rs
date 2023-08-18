use crate::{TextureBindGroup, Uniform};
use crate::image::renderer::{ImageFade, ImageName};

pub(crate) struct ImageRenderGroup {
    pub(crate) image_name: ImageName,
    pub(crate) fade_bind_group: wgpu::BindGroup,
    pub(crate) fade_uniform: Uniform<ImageFade>,
}
