use crate::color::Color;
use crate::coord::{Area, Depth, Position};
use crate::instance::AttributeExtractor;
use crate::text::rasterization;
use crate::text::request::RequestData;

impl AttributeExtractor<RequestData> for Position {
    fn extract(request: &RequestData) -> Self {
        request.position
    }
}
impl AttributeExtractor<RequestData> for Area {
    fn extract(request: &RequestData) -> Self {
        request.area
    }
}
impl AttributeExtractor<RequestData> for Depth {
    fn extract(request: &RequestData) -> Self {
        request.depth
    }
}
impl AttributeExtractor<RequestData> for Color {
    fn extract(request: &RequestData) -> Self {
        request.color
    }
}
impl AttributeExtractor<RequestData> for rasterization::Descriptor {
    fn extract(request: &RequestData) -> Self {
        request.descriptor.expect("no descriptor attached")
    }
}
