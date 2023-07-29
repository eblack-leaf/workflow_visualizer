pub use attachment::TextAttachment;
pub(crate) use component::{Cache, Difference};
pub use component::{
    Text, TextGridLocation, TextGridPlacement, TextLetterDimensions, TextLineStructure, TextScale,
    TextScaleAlignment, TextTag, TextValue, TextWrapStyle,
};
pub(crate) use font::{AlignedFonts, MonoSpacedFont};
pub(crate) use system::{color_diff, scale_change};

mod atlas;
mod attachment;
mod component;
mod font;
mod render_group;
mod renderer;
mod system;
