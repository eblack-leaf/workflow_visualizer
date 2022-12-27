use crate::color::Color;
use crate::coord::{Area, Depth, Position};
use crate::text::attribute::buffer::Buffer;
use crate::text::attribute::instance::GlyphKey;
use crate::text::attribute::Coordinator;
use crate::text::rasterization;
use crate::text::rasterization::{GlyphHash, Rasterization};
use crate::text::scale::Scale;
use anymap::AnyMap;
use bevy_ecs::prelude::Entity;

pub(crate) struct Add {
    pub(crate) index_key: GlyphKey,
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
        add_attr(&mut coordinator.attribute_adds, add.position);
        add_attr(&mut coordinator.attribute_adds, add.area);
        add_attr(&mut coordinator.attribute_adds, add.area);
        add_attr(&mut coordinator.attribute_adds, add.area);
        coordinator
            .rasterization_requests
            .push(rasterization::Add::new(add.hash, add.character, add.scale));
        coordinator
            .rasterization_response_listeners
            .insert((instance, add.hash));
        coordinator
            .indexer_responses
            .push((add.index_key, instance).into());
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
pub(crate) fn push_rasterization_requests(
    coordinator: &mut Coordinator,
    rasterization: &mut Rasterization,
) {
    rasterization.adds = coordinator.rasterization_requests.drain(..).collect();
}
pub(crate) fn read_rasterizations(coordinator: &mut Coordinator, rasterization: &Rasterization) {
    for listener in coordinator.rasterization_response_listeners.iter() {
        let placement = rasterization
            .placements
            .get(*rasterization.placement_order.get(&listener.1).unwrap())
            .unwrap();
        add_attr(&mut coordinator.attribute_adds, placement.placement);
    }
}
