pub use attachment::TextAttachment;

pub use component::{
    Text, TextGridLocation, TextGridPlacement, TextLetterDimensions, TextLineStructure, TextScale,
    TextTag, TextValue, TextWrapStyle,
};
pub use font::{
    KnownTextDimension, MonoSpacedFont, TextSectionDescriptor, TextSectionDescriptorKnown,
};

mod atlas;
mod attachment;
mod component;
mod font;
mod render_group;
mod renderer;
mod system;
