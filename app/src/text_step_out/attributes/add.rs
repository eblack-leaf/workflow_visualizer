use bevy_ecs::prelude::{Commands, Component, Entity, Res, ResMut};

use crate::color::Color;
use crate::coord::{Area, Depth, Position};
use crate::text_step_out::attributes::buffers::{attribute_size, CpuAttributes};
use crate::text_step_out::attributes::write::{Write, Writes};
use crate::text_step_out::attributes::{Coordinator, GpuAttributes, Index};
use crate::text_step_out::rasterization::placement::RasterizationPlacement;
use crate::text_step_out::rasterization::{
    RasterizationRequest, RasterizationRequestCallPoint, Rasterizations, RasterizedGlyphHash,
};
use crate::text_step_out::scale::Scale;
#[derive(Component)]
pub struct IndexResponse {
    pub entity: Entity,
    pub index: Index,
}
impl IndexResponse {
    pub fn new(entity: Entity, index: Index) -> Self {
        Self { entity, index }
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
        Self { to_add: Vec::new() }
    }
}

pub fn setup_added_instances(mut cmd: Commands) {
    cmd.insert_resource(AddedInstances::new());
}

pub struct Add<Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync> {
    pub index: Index,
    pub attribute: Attribute,
}

impl<Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync> Add<Attribute> {
    pub fn new(index: Index, attribute: Attribute) -> Self {
        Self { index, attribute }
    }
}

pub struct Adds<Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync> {
    pub adds: Vec<Add<Attribute>>,
}

impl<Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync> Adds<Attribute> {
    pub fn new() -> Self {
        Self { adds: Vec::new() }
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
    coordinator.growth = Option::from(added_instances.to_add.len() as u32);
    added_instances
        .to_add
        .drain(..)
        .for_each(|added_instance: AddedInstance| {
            let index = coordinator.generate_index();
            positions
                .adds
                .push(Add::new(index, added_instance.add_data.position));
            areas
                .adds
                .push(Add::new(index, added_instance.add_data.area));
            depths
                .adds
                .push(Add::new(index, added_instance.add_data.depth));
            colors
                .adds
                .push(Add::new(index, added_instance.add_data.color));
            cmd.spawn()
                .insert(RasterizationRequest::new(
                    RasterizationRequestCallPoint::Add,
                    added_instance.entity,
                    added_instance.add_data.hash,
                    added_instance.add_data.character,
                    added_instance.add_data.scale,
                    index,
                ))
                .insert(IndexResponse::new(added_instance.entity, index));
        });
}

pub fn add_cpu_attrs<Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync>(
    adds: ResMut<Adds<Attribute>>,
    mut cpu_attributes: ResMut<CpuAttributes<Attribute>>,
) {
    for add in adds.adds.iter() {
        *cpu_attributes
            .attributes
            .get_mut(add.index.0 as usize)
            .unwrap() = add.attribute;
    }
}

pub fn add_gpu_attrs<Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync>(
    queue: Res<wgpu::Queue>,
    attributes: Res<GpuAttributes<Attribute>>,
    mut adds: ResMut<Adds<Attribute>>,
) {
    for add in adds.adds.drain(..) {
        queue.write_buffer(
            &attributes.buffer,
            attribute_size::<Attribute>(add.index.0 as u32),
            bytemuck::cast_slice(&[add.attribute]),
        );
    }
}

pub fn growth(
    mut coordinator: ResMut<Coordinator>,
    mut position_attributes: ResMut<CpuAttributes<Position>>,
    mut area_attributes: ResMut<CpuAttributes<Area>>,
    mut depth_attributes: ResMut<CpuAttributes<Depth>>,
    mut color_attributes: ResMut<CpuAttributes<Color>>,
    mut rasterization_placement_attributes: ResMut<CpuAttributes<RasterizationPlacement>>,
    mut positions: ResMut<GpuAttributes<Position>>,
    mut areas: ResMut<GpuAttributes<Area>>,
    mut depths: ResMut<GpuAttributes<Depth>>,
    mut colors: ResMut<GpuAttributes<Color>>,
    mut rasterization_placements: ResMut<GpuAttributes<RasterizationPlacement>>,
    queue: Res<wgpu::Queue>,
    device: Res<wgpu::Device>,
) {
    if let Some(growth) = coordinator.growth {
        if coordinator.current + growth > coordinator.max {
            let buffer_growth = coordinator.max.abs_diff(coordinator.current + growth);
            // TODO change to .resize(len, value)
            position_attributes
                .attributes
                .reserve(buffer_growth as usize);
            area_attributes.attributes.reserve(buffer_growth as usize);
            depth_attributes.attributes.reserve(buffer_growth as usize);
            color_attributes.attributes.reserve(buffer_growth as usize);
            rasterization_placement_attributes
                .attributes
                .reserve(buffer_growth as usize);
            // grow gpu buffers
            positions.size += attribute_size::<Position>(buffer_growth);
            areas.size += attribute_size::<Area>(buffer_growth);
            depths.size += attribute_size::<Depth>(buffer_growth);
            colors.size += attribute_size::<Color>(buffer_growth);
            rasterization_placements.size +=
                attribute_size::<RasterizationPlacement>(buffer_growth);
            *positions = GpuAttributes::<Position>::new(&device, positions.size as u32);
            *areas = GpuAttributes::<Area>::new(&device, areas.size as u32);
            *depths = GpuAttributes::<Depth>::new(&device, depths.size as u32);
            *colors = GpuAttributes::<Color>::new(&device, colors.size as u32);
            *rasterization_placements = GpuAttributes::<RasterizationPlacement>::new(
                &device,
                rasterization_placements.size as u32,
            );
            // write current state to gpu buffers so adds can write normally on top of it
            queue.write_buffer(
                &positions.buffer,
                0,
                bytemuck::cast_slice(&position_attributes.attributes),
            );
            queue.write_buffer(
                &areas.buffer,
                0,
                bytemuck::cast_slice(&area_attributes.attributes),
            );
            queue.write_buffer(
                &depths.buffer,
                0,
                bytemuck::cast_slice(&depth_attributes.attributes),
            );
            queue.write_buffer(
                &colors.buffer,
                0,
                bytemuck::cast_slice(&color_attributes.attributes),
            );
            queue.write_buffer(
                &rasterization_placements.buffer,
                0,
                bytemuck::cast_slice(&rasterization_placement_attributes.attributes),
            );
        }
    }
}
