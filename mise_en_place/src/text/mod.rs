pub use crate::text::render_group::TextBound;
pub use crate::text::renderer::TextRenderer;
pub use crate::text::scale::TextScaleAlignment;
pub use crate::text::text::{Text, TextBundle};

mod atlas;
mod backend_system;
mod cache;
mod frontend_system;
mod coords;
mod cpu_buffer;
mod difference;
mod extraction;
mod font;
mod glyph;
mod gpu_buffer;
mod index;
mod null_bit;
mod place;
mod render_group;
mod renderer;
mod scale;
mod text;
mod vertex;

