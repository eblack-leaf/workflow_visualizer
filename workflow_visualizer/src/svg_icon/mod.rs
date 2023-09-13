mod attachment;
mod interface;
mod render_group;
mod renderer;
mod svg_glue;
mod tessellation;
pub use crate::icon_scaling::IconScale;
pub(crate) use attachment::SvgIconAttachment;
pub use interface::{SvgIcon, SvgTag};
pub use tessellation::{BundledSvg, SvgRequest, TessellatedSvg};

pub type SvgData = Vec<u8>;
