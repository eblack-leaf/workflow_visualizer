pub(crate) use render_group::TextBound;

pub use crate::text::attachment::TextAttachment;
pub use crate::text::place::{WrapStyleComponent, WrapStyleExpt};
pub use crate::text::render_group::TextGridGuide;
pub(crate) use crate::text::scale::AlignedFonts;
pub(crate) use crate::text::scale::TextScale;
pub use crate::text::scale::TextScaleAlignment;
pub use crate::text::scale::TextScaleLetterDimensions;
pub use crate::text::text::{
    Letter, LetterStyle, TextBuffer, TextBundle, TextContent, TextContentView, TextLineStructure,
};

mod atlas;
mod attachment;
mod backend_system;
mod cache;
mod coords;
mod difference;
mod extraction;
mod font;
mod frontend_system;
mod glyph;
mod place;
mod render_group;
mod renderer;
mod scale;
mod text;
mod vertex;
