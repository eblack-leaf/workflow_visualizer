pub use attachment::TextAttachment;
pub use component::{
    Text, TextGridLocation, TextGridPlacement, TextLetterDimensions, TextLineStructure,
    TextRequest, TextScale, TextScaleAlignment, TextWrapStyle, WrapStyleExpt,
};
pub(crate) use component::Difference;
pub(crate) use font::{AlignedFonts, MonoSpacedFont};
pub(crate) use system::{color_diff, scale_change};

mod atlas;
mod attachment;
mod component;
mod font;
mod render_group;
mod renderer;
mod system;
