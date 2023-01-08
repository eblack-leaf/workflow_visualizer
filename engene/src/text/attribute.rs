use crate::color::Color;
use crate::coord::{Area, Depth, Position};
use crate::instance::AttributeHandler;
use crate::text::{rasterization, Request};
use bytemuck::{Pod, Zeroable};

impl AttributeHandler<Request> for Position {
    type Attribute = Self;
    fn extract(request: &Request) -> Self::Attribute {
        request.position
    }
}
impl AttributeHandler<Request> for Area {
    type Attribute = Self;
    fn extract(request: &Request) -> Self::Attribute {
        request.area
    }
}
impl AttributeHandler<Request> for Depth {
    type Attribute = Self;
    fn extract(request: &Request) -> Self::Attribute {
        request.depth
    }
}
impl AttributeHandler<Request> for Color {
    type Attribute = Self;
    fn extract(request: &Request) -> Self::Attribute {
        request.color
    }
}
impl AttributeHandler<Request> for rasterization::Descriptor {
    type Attribute = Self;
    fn extract(request: &Request) -> Self::Attribute {
        request.descriptor.expect("no descriptor attached")
    }
}
