pub use crate::text::renderer::TextRenderer;
pub use crate::text::scale::Scale;
pub use crate::text::text::{Text, TextBundle};

mod atlas;
mod attach;
mod cache;
mod compute_system;
mod coords;
mod difference;
mod extraction;
mod font;
mod glyph;
mod index;
mod place;
mod render_group;
mod render_system;
mod renderer;
mod scale;
mod text;
mod vertex;

