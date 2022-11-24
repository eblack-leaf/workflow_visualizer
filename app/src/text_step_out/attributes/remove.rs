use crate::text_step_out::attributes::buffers::CpuAttributes;
use crate::text_step_out::attributes::write::{Write, Writes};
use crate::text_step_out::attributes::{Coordinator, Index};
use crate::text_step_out::rasterization::references::RasterizationReferences;
use crate::text_step_out::rasterization::RasterizedGlyphHash;
use crate::{Area, Color, Depth, Position, RasterizationPlacement};
use bevy_ecs::prelude::ResMut;

pub struct Swap {
    pub old: Index,
    pub new: Index,
}
impl Swap {
    pub fn new(old: Index, new: Index) -> Self {
        Self { old, new }
    }
}
// for app.post_processing() to read back to compute
pub struct Swaps {
    pub swaps: Vec<Swap>,
}
impl Swaps {
    pub fn new() -> Self {
        Self { swaps: Vec::new() }
    }
}
pub struct Remove {
    pub index: Index,
    pub rasterization_hash: RasterizedGlyphHash,
}
pub struct Removes {
    pub to_remove: Vec<Remove>,
}
impl Removes {
    pub fn new() -> Self {
        Self {
            to_remove: Vec::new(),
        }
    }
}
pub fn remove_instances(
    mut coordinator: ResMut<Coordinator>,
    mut removed_instances: ResMut<Removes>,
    mut swaps: ResMut<Swaps>,
    mut references: ResMut<RasterizationReferences>,
    mut position_writes: ResMut<Writes<Position>>,
    mut position_attrs: ResMut<CpuAttributes<Position>>,
    mut area_writes: ResMut<Writes<Area>>,
    mut area_attrs: ResMut<CpuAttributes<Area>>,
    mut depth_writes: ResMut<Writes<Depth>>,
    mut depth_attrs: ResMut<CpuAttributes<Depth>>,
    mut color_writes: ResMut<Writes<Color>>,
    mut color_attrs: ResMut<CpuAttributes<Color>>,
    mut rasterization_placement_writes: ResMut<Writes<RasterizationPlacement>>,
    mut rasterization_placement_attrs: ResMut<CpuAttributes<RasterizationPlacement>>,
) {
    removed_instances.to_remove.iter().for_each(|remove| {
        references.remove_ref(remove.rasterization_hash);
        swaps
            .swaps
            .push(Swap::new(coordinator.current_index(), remove.index));
        position_writes.writes.push(Write::new(
            remove.index,
            *position_attrs
                .attributes
                .get(coordinator.current_index().0 as usize)
                .unwrap(),
        ));
        position_attrs
            .attributes
            .remove(coordinator.current_index().0 as usize);
        area_writes.writes.push(Write::new(
            remove.index,
            *area_attrs
                .attributes
                .get(coordinator.current_index().0 as usize)
                .unwrap(),
        ));
        area_attrs
            .attributes
            .remove(coordinator.current_index().0 as usize);
        depth_writes.writes.push(Write::new(
            remove.index,
            *depth_attrs
                .attributes
                .get(coordinator.current_index().0 as usize)
                .unwrap(),
        ));
        depth_attrs
            .attributes
            .remove(coordinator.current_index().0 as usize);
        color_writes.writes.push(Write::new(
            remove.index,
            *color_attrs
                .attributes
                .get(coordinator.current_index().0 as usize)
                .unwrap(),
        ));
        color_attrs
            .attributes
            .remove(coordinator.current_index().0 as usize);
        rasterization_placement_writes.writes.push(Write::new(
            remove.index,
            *rasterization_placement_attrs
                .attributes
                .get(coordinator.current_index().0 as usize)
                .unwrap(),
        ));
        rasterization_placement_attrs
            .attributes
            .remove(coordinator.current_index().0 as usize);
        coordinator.shrink();
    });
}
