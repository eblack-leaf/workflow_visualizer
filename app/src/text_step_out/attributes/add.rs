use bevy_ecs::prelude::{Commands, Entity, Res, ResMut};

use crate::color::Color;
use crate::coord::{Area, Depth, Position};
use crate::text_step_out::attributes::write::{Write, Writes};
use crate::text_step_out::attributes::{Coordinator, GpuAttributes, Index};
use crate::text_step_out::attributes::buffers::{attribute_size, CpuAttributes};
use crate::text_step_out::rasterization::placement::RasterizationPlacement;
use crate::text_step_out::rasterization::{WriteRasterizationRequest, Rasterizations, RasterizedGlyphHash, AddRasterizationRequest};
use crate::text_step_out::scale::Scale;
pub struct IndexResponse {
    pub entity: Entity,
    pub index: Index,
}
impl IndexResponse {
    pub fn new(entity: Entity, index: Index) -> Self {
        Self {
            entity,
            index,
        }
    }
}
pub struct AddData {
    pub character: char,
    pub hash: RasterizedGlyphHash,
    pub scale: Scale,
    pub position: Position,
    pub area: Area,
    pub depth: Depth,
    pub color: Color,
}
pub struct AddedInstance {
    pub entity: Entity,
    pub add_data: AddData,
}
pub struct AddedInstances {
    pub to_add: Vec<AddedInstance>,
}
impl AddedInstances {
    pub fn new() -> Self {
        Self {
            to_add: Vec::new()
        }
    }
}
pub fn setup_added_instances(mut cmd: Commands) {
    cmd.insert_resource(AddedInstances::new());
}
pub struct Add<Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone> {
    pub index: Index,
    pub attribute: Attribute,
}
impl<Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone> Add<Attribute> {
    pub fn new(index: Index, attribute: Attribute) -> Self {
        Self { index, attribute }
    }
}
pub struct Adds<Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone> {
    pub adds: Vec<Add<Attribute>>,
}
impl<Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone> Adds<Attribute> {
    pub fn new() -> Self {
        Self {
            adds: Vec::new(),
        }
    }
}
pub fn add_instances(
    mut coordinator: ResMut<Coordinator>,
    mut added_instances: ResMut<AddedInstances>,
    mut positions: ResMut<Adds<Position>>,
    mut areas: ResMut<Adds<Area>>,
    mut depths: ResMut<Adds<Depth>>,
    mut colors: ResMut<Adds<Color>>,
    mut cmd: Commands,
) {
    added_instances.to_add.drain(..).for_each(|added_instance: AddedInstance| {
        let index = coordinator.generate_index();
        positions
            .adds
            .push(Add::new(index, added_instance.add_data.position));
        areas.adds.push(Add::new(index, added_instance.add_data.area));
        depths.adds.push(Add::new(index, added_instance.add_data.depth));
        colors.adds.push(Add::new(index, added_instance.add_data.color));
        cmd.spawn().insert(AddRasterizationRequest::new(
            added_instance.entity,
            added_instance.add_data.hash,
            added_instance.add_data.character,
            added_instance.add_data.scale,
            index,
        )).insert(IndexResponse::new(added_instance.entity, index));
    });
}
pub fn add_attributes<Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone>(adds: ResMut<Adds<Attribute>>,
                      attributes: Res<GpuAttributes<Attribute>>,
                      queue: Res<wgpu::Queue>,
                      mut cpu_attributes: ResMut<CpuAttributes<Attribute>>,) {
    for add in adds.adds.drain(..) {
        *cpu_attributes.attributes.get_mut(add.index.0 as usize) = add.attribute;
        queue.write_buffer(
            &attributes.buffer,
            attribute_size::<Attribute>(add.index.0 as u32),
            bytemuck::cast_slice(&add.attribute),
        );
    }
}
pub fn growth(
    mut coordinator: ResMut<Coordinator>,
    mut adds: ResMut<AddedInstances>,
    mut position_attributes: ResMut<CpuAttributes<Position>>,
    mut area_attributes: ResMut<CpuAttributes<Position>>,
    mut depth_attributes: ResMut<CpuAttributes<Position>>,
    mut color_attributes: ResMut<CpuAttributes<Position>>,
    mut rasterization_placement_attributes: ResMut<CpuAttributes<Position>>,
    mut positions: ResMut<GpuAttributes<Position>>,
    mut areas: ResMut<GpuAttributes<Area>>,
    mut depths: ResMut<GpuAttributes<Depth>>,
    mut colors: ResMut<GpuAttributes<Color>>,
    mut rasterization_placements: ResMut<GpuAttributes<RasterizationPlacement>>,
    mut positions_writes: ResMut<Writes<Position>>,
    mut areas_writes: ResMut<Writes<Area>>,
    mut depths_writes: ResMut<Writes<Depth>>,
    mut colors_writes: ResMut<Writes<Color>>,
    mut rasterization_placement_writes: ResMut<Writes<Color>>,
    queue: Res<wgpu::Queue>,
    device: Res<wgpu::Device>,
) {
    let growth = adds.to_add.len();
    let buffer_growth = 0;
    if coordinator.current + growth > coordinator.max {
        position_attributes.attributes.reserve(buffer_growth as usize);
        area_attributes.attributes.reserve(buffer_growth as usize);
        depth_attributes.attributes.reserve(buffer_growth as usize);
        color_attributes.attributes.reserve(buffer_growth as usize);
        // grow gpu buffers
        // drain writes into cpu buffers
        // write current state to gpu buffers so adds can write normally on top of it
    }
}
