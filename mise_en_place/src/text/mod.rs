pub(crate) use render_group::TextBound;

pub use crate::text::attachment::TextAttachment;
pub use crate::text::attachment::TextStages;
pub use crate::text::place::{WrapStyleComponent, WrapStyleExpt};
pub use crate::text::render_group::TextBoundGuide;
pub use crate::text::scale::TextScaleAlignment;
pub use crate::text::text::{PartitionMetadata, Text, TextBundle, TextPartition};

mod atlas;
mod backend_system;
mod cache;
mod coords;
mod difference;
mod extraction;
mod font;
mod frontend_system;
mod glyph;
mod place;
mod attachment;
mod render_group;
mod renderer;
mod scale;
mod text;
mod vertex;
