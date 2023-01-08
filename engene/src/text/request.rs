use crate::color::Color;
use crate::coord::{Area, Depth, Position};
use crate::text::scale::Scale;
use crate::text::{rasterization, GlyphHash};

pub struct Request {
    pub character: char,
    pub scale: Scale,
    pub hash: GlyphHash,
    pub position: Position,
    pub area: Area,
    pub depth: Depth,
    pub color: Color,
    pub descriptor: Option<rasterization::Descriptor>,
}
