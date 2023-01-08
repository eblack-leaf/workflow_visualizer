use crate::color::Color;
use crate::coord::{Area, Depth, Position};
use crate::instance::AttributeHandler;
use crate::text::rasterization;
use crate::text::request::Request;

impl AttributeHandler<Request> for Position {
    fn extract(request: &Request) -> Self {
        request.position
    }
}
impl AttributeHandler<Request> for Area {
    fn extract(request: &Request) -> Self {
        request.area
    }
}
impl AttributeHandler<Request> for Depth {
    fn extract(request: &Request) -> Self {
        request.depth
    }
}
impl AttributeHandler<Request> for Color {
    fn extract(request: &Request) -> Self {
        request.color
    }
}
impl AttributeHandler<Request> for rasterization::Descriptor {
    fn extract(request: &Request) -> Self {
        request.descriptor.expect("no descriptor attached")
    }
}
