use bevy_ecs::prelude::{Entity, Res, ResMut};

use crate::color::Color;
use crate::coord::{Area, Depth, Position};
use crate::text_refactor::instances::Index;
use crate::text_step_out::attributes::writes::Writes;
use crate::text_step_out::attributes::{AttributeBuffer, Coordinator};
use crate::text_step_out::instance::Instance;
use crate::text_step_out::rasterization::placement::RasterizationPlacement;
use crate::text_step_out::rasterization::Rasterizations;

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
    mut rasterizations: ResMut<Rasterizations>,
    mut coordinator: ResMut<Coordinator>,
    mut adds: ResMut<Adds>,
    mut positions: ResMut<Writes<Position>>,
    mut areas: ResMut<Writes<Area>>,
    mut depths: ResMut<Writes<Depth>>,
    mut colors: ResMut<Writes<Color>>,
    mut rasterization_placements: ResMut<Writes<RasterizationPlacement>>,
    mut positions_buffer: ResMut<AttributeBuffer<Position>>,
    mut areas_buffer: ResMut<AttributeBuffer<Area>>,
    mut depths_buffer: ResMut<AttributeBuffer<Depth>>,
    mut colors_buffer: ResMut<AttributeBuffer<Color>>,
    mut rasterization_placements_buffer: ResMut<AttributeBuffer<RasterizationPlacement>>,
) {
    let growth = adds.to_add.len();
    let mut instances = Vec::<Instance>::new();
    adds.to_add.drain(..).for_each(|add: Add| {
        let rasterization_placement = 0;// cant do here, must send request to make that work
        // the request handler will check size and do shrinking before giving out
        let index = 0;
        // add to instances
    });
    if coordinator.current + growth > coordinator.max {
        // grow all buffers by remaking with ::new(new_max)
        // using cpu_attrs to fill + added
        // shortcut with create_buffer_init
        // consume writes
        return;
    }
    // otherwise process adds
    // write indexes back to buffer by entity using InstanceAddResponse
    // make write<attr> for the new indexes
}
