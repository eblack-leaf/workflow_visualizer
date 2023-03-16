use crate::content_panel::vertex::CORNER_DEPTH;
use crate::content_panel::{Cache, ContentArea, Difference, Extraction, Padding};
use crate::{Area, Color, InterfaceContext, Layer, Position};
use bevy_ecs::change_detection::ResMut;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{Changed, Commands, Query};

pub(crate) fn pull_differences(
    mut extraction: ResMut<Extraction>,
    mut differential: Query<(Entity, &mut Difference), Changed<Difference>>,
) {
    for (entity, mut difference) in differential.iter_mut() {
        extraction.differences.insert(entity, difference.clone());
        *difference = Difference::new();
    }
}

pub fn calc_area_from_content_area(
    mut content_changed: Query<
        (&ContentArea, &Padding, &mut Area<InterfaceContext>),
        Changed<ContentArea>,
    >,
) {
    for (content_area, padding, mut area) in content_changed.iter_mut() {
        let calculated_area = content_area.0
            + padding.0 * Area::from((2, 2))
            + Area::from((CORNER_DEPTH * 2, CORNER_DEPTH * 2));
        *area = calculated_area;
    }
}

pub(crate) fn position_diff(
    mut pos_changed: Query<
        (&Position<InterfaceContext>, &mut Cache, &mut Difference),
        Changed<Position<InterfaceContext>>,
    >,
) {
}

pub(crate) fn content_area_diff(
    mut content_area_changed: Query<
        (&ContentArea, &mut Cache, &mut Difference),
        Changed<ContentArea>,
    >,
) {
}

pub(crate) fn layer_diff(
    mut layer_changed: Query<(&Layer, &mut Cache, &mut Difference), Changed<Layer>>,
) {
}

pub(crate) fn color_diff(
    mut color_changed: Query<(&Color, &mut Cache, &mut Difference), Changed<Color>>,
) {
}
