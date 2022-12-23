use crate::color::Color;
use crate::coord::{Area, Depth, Position};
use crate::text::attribute::buffer::Buffer;
use crate::text::attribute::Coordinator;
use crate::text::rasterization;
use crate::text::rasterization::GlyphHash;
use crate::text::scale::Scale;
use anymap::AnyMap;
pub(crate) struct Add {
    pub(crate) character: char,
    pub(crate) scale: Scale,
    pub(crate) hash: GlyphHash,
    pub(crate) position: Position,
    pub(crate) area: Area,
    pub(crate) depth: Depth,
    pub(crate) color: Color,
}
pub(crate) fn add(coordinator: &mut Coordinator) {
    for add in coordinator.adds.iter() {
        let instance = coordinator.indexer.next();
        add_attr::<Position>(&mut coordinator.attribute_adds, add.position);
        add_attr::<Area>(&mut coordinator.attribute_adds, add.area);
        add_attr::<Area>(&mut coordinator.attribute_adds, add.area);
        add_attr::<Area>(&mut coordinator.attribute_adds, add.area);
        coordinator
            .rasterization_requests
            .push(rasterization::Add::new(add.hash, add.character, add.scale));
        coordinator
            .rasterization_response_listeners
            .insert((instance, add.hash));
    }
}
fn add_attr<
    Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default,
>(
    attr_adds: &mut AnyMap,
    attr: Attribute,
) {
    attr_adds.get_mut::<Vec<Attribute>>().unwrap().push(attr);
}
