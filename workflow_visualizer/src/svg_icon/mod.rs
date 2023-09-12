mod attachment;
mod interface;
mod render_group;
mod renderer;
mod svg_glue;
mod tessellation;
pub(crate) use attachment::SvgIconAttachment;
pub use interface::{SvgIcon, SvgIconScale, SvgTag};
pub use tessellation::{BundledSvg, SvgRequest, TessellatedSvg};

pub type SvgData = Vec<u8>;
