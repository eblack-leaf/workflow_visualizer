use crate::Position;

pub(crate) const GLYPH_AABB: [Vertex; 6] = [
    Vertex::new(Position::new(0.0, 0.0)),
    Vertex::new(Position::new(0.0, 1.0)),
    Vertex::new(Position::new(1.0, 0.0)),
    Vertex::new(Position::new(1.0, 0.0)),
    Vertex::new(Position::new(0.0, 1.0)),
    Vertex::new(Position::new(1.0, 1.0)),
];

#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Copy, Clone)]
pub(crate) struct Vertex {
    pub position: Position,
}

impl Vertex {
    pub const fn new(position: Position) -> Self {
        Self { position }
    }
}
