use crate::color::Color;
use crate::coord::{Area, Depth, Position};
use crate::instance::AttributeHandler;
use crate::text::rasterization;
use crate::text::request::RequestData;

impl AttributeHandler<RequestData> for Position {
    fn extract(request: &RequestData) -> Self {
        request.position
    }
    fn null() -> Self {
        Self::default()
    }
}
impl AttributeHandler<RequestData> for Area {
    fn extract(request: &RequestData) -> Self {
        request.area
    }
    fn null() -> Self {
        Self::default()
    }
}
impl AttributeHandler<RequestData> for Depth {
    fn extract(request: &RequestData) -> Self {
        request.depth
    }
    fn null() -> Self {
        Self::default()
    }
}
impl AttributeHandler<RequestData> for Color {
    fn extract(request: &RequestData) -> Self {
        request.color
    }
    fn null() -> Self {
        Self::default()
    }
}
impl AttributeHandler<RequestData> for rasterization::Descriptor {
    fn extract(request: &RequestData) -> Self {
        request.descriptor.expect("no descriptor attached")
    }
    fn null() -> Self {
        Self::default()
    }
}
