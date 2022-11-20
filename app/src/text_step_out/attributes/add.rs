use bevy_ecs::prelude::{Entity, Res, ResMut};

use crate::color::Color;
use crate::coord::{Area, Depth, Position};
use crate::text_refactor::instances::Index;
use crate::text_step_out::attributes::writes::Writes;
use crate::text_step_out::attributes::Coordinator;
use crate::text_step_out::instance::Instance;
use crate::text_step_out::rasterization::placement::RasterizationPlacement;

pub struct ResponseData {
    pub index: Index,
    pub rasterization_placement: RasterizationPlacement,
}

pub struct Response {
    pub entity: Entity,
    pub index: Index,
    pub response_data: ResponseData,
}

pub struct Responses {
    pub responses: Vec<Response>,
}

pub struct AddData {
    pub character: char,
    // whatever else provided by compute side
}

pub struct Add {
    pub entity: Entity,
    pub add_data: AddData,
}

pub struct Adds {
    pub to_add: Vec<Add>,
}

// run after removes
pub fn add_instances(
    mut coordinator: ResMut<Coordinator>,
    mut added_instances: ResMut<Adds>,
    mut positions: ResMut<Writes<Position>>,
    mut areas: ResMut<Writes<Area>>,
    mut depths: ResMut<Writes<Depth>>,
    mut colors: ResMut<Writes<Color>>,
    mut rasterization_placements: ResMut<Writes<RasterizationPlacement>>,
) {
    let growth = added_instances.to_add.len();
    if coordinator.current + growth > coordinator.max {
        // grow all buffers by remaking with ::new(new_max)
        // using cpu_attrs to fill + added
    }
    // write indexes back to buffer by entity using InstanceAddResponse
    // make write<attr> for the new indexes
}
