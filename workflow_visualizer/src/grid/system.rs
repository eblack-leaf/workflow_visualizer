use std::collections::HashMap;

use bevy_ecs::change_detection::{Res, ResMut};
use bevy_ecs::event::EventReader;
use bevy_ecs::prelude::{Changed, Local, Query, Without};
use tracing::trace;

use crate::grid::responsive::ResponsiveGridPoint;
use crate::grid::AbsolutePlacement;
use crate::{
    Area, DiagnosticsHandle, Grid, HorizontalSpan, InterfaceContext, Position, Record,
    ResponsiveGridView, ViewportHandle, WindowResize,
};

fn update_section(
    grid: &Grid,
    view: &ResponsiveGridView,
    pos: &mut Position<InterfaceContext>,
    area: &mut Area<InterfaceContext>,
) {
    let section = grid.calc_section_from_responsive(view);
    *pos = section.position;
    *area = section.area;
}

#[derive(Default)]
pub(crate) struct GridConfigRecorder {
    times_span_configured: HashMap<HorizontalSpan, usize>,
}

impl GridConfigRecorder {
    pub(crate) fn record_span_configured(&mut self, span: HorizontalSpan) {
        if let Some(count) = self.times_span_configured.get_mut(&span) {
            *count += 1;
        } else {
            self.times_span_configured.insert(span, 1);
        }
    }
}

impl Record for GridConfigRecorder {
    fn record(&self, core_record: String) -> String {
        format!("{:?}:{:?}", core_record, self.times_span_configured)
    }
}

pub(crate) fn config_grid(
    viewport_handle: Res<ViewportHandle>,
    window_resize_events: EventReader<WindowResize>,
    mut responsive: Query<
        (
            &ResponsiveGridView,
            &mut Position<InterfaceContext>,
            &mut Area<InterfaceContext>,
        ),
        (Without<ResponsiveGridPoint>, Without<AbsolutePlacement>),
    >,
    mut responsive_points: Query<
        (&ResponsiveGridPoint, &mut Position<InterfaceContext>),
        (Without<ResponsiveGridView>, Without<AbsolutePlacement>),
    >,
    mut absolutes: Query<
        (
            &AbsolutePlacement,
            &mut Position<InterfaceContext>,
            &mut Area<InterfaceContext>,
        ),
        (Without<ResponsiveGridView>, Without<ResponsiveGridPoint>),
    >,
    mut grid: ResMut<Grid>,
    #[cfg(feature = "diagnostics")] mut diagnostics: Local<DiagnosticsHandle<GridConfigRecorder>>,
) {
    if !window_resize_events.is_empty() {
        // configure grid configs + span
        *grid = Grid::new(viewport_handle.section.area);
        #[cfg(feature = "diagnostics")]
        diagnostics.ext.record_span_configured(grid.span);
        // update all views
        for (view, mut pos, mut area) in responsive.iter_mut() {
            update_section(grid.as_ref(), view, pos.as_mut(), area.as_mut());
        }
        for (view, mut pos) in responsive_points.iter_mut() {
            let x = grid
                .calc_horizontal_location(view.mapping.get(&grid.span).expect("mapping").x)
                .to_pixel();
            let y = grid
                .calc_horizontal_location(view.mapping.get(&grid.span).expect("mapping").y)
                .to_pixel();
            *pos = Position::new(x, y);
        }
        for (placement, mut pos, mut area) in absolutes.iter_mut() {
            *pos = placement.0.position;
            *area = placement.0.area;
        }
        #[cfg(feature = "diagnostics")]
        trace!("{:?}", diagnostics.record());
    }
}

pub(crate) fn set_point_from_view(
    grid: Res<Grid>,
    mut changed: Query<
        (&ResponsiveGridPoint, &mut Position<InterfaceContext>),
        Changed<ResponsiveGridPoint>,
    >,
) {
    for (view, mut pos) in changed.iter_mut() {
        let x = grid
            .calc_horizontal_location(view.mapping.get(&grid.span).expect("mapping").x)
            .to_pixel();
        let y = grid
            .calc_horizontal_location(view.mapping.get(&grid.span).expect("mapping").y)
            .to_pixel();
        *pos = Position::new(x, y);
    }
}

pub(crate) fn set_from_view(
    grid: Res<Grid>,
    mut changed: Query<
        (
            &ResponsiveGridView,
            &mut Position<InterfaceContext>,
            &mut Area<InterfaceContext>,
        ),
        Changed<ResponsiveGridView>,
    >,
) {
    for (view, mut pos, mut area) in changed.iter_mut() {
        update_section(grid.as_ref(), view, pos.as_mut(), area.as_mut());
    }
}

pub(crate) fn set_from_absolute(
    grid: Res<Grid>,
    mut changed: Query<
        (
            &AbsolutePlacement,
            &mut Position<InterfaceContext>,
            &mut Area<InterfaceContext>,
        ),
        Changed<AbsolutePlacement>,
    >,
) {
    for (placement, mut pos, mut area) in changed.iter_mut() {
        *pos = placement.0.position;
        *area = placement.0.area;
    }
}
