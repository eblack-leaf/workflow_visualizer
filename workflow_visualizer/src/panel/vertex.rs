use std::f32::consts::{FRAC_PI_2, PI};

use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

use crate::gfx::GfxSurface;
use crate::{DeviceContext, InterfaceContext, Interpolator, Panel, Position, RawPosition};

#[repr(C)]
#[derive(Pod, Zeroable, Copy, Clone, Default, Debug)]
pub struct ListenOffset {
    pub listen_x: f32,
    pub listen_y: f32,
}
impl ListenOffset {
    pub const LISTEN_ON: f32 = 1.0;
    pub const LISTEN_OFF: f32 = 0.0;
    pub fn new(listen_x: f32, listen_y: f32) -> Self {
        Self { listen_x, listen_y }
    }
    pub fn from_bool(b: bool) -> f32 {
        match b {
            true => Self::LISTEN_ON,
            false => Self::LISTEN_OFF,
        }
    }
}
#[repr(C)]
#[derive(Pod, Zeroable, Copy, Clone, Default, Debug)]
pub struct PanelVertex {
    pub position: RawPosition,
    pub listen_offset: ListenOffset,
}
impl PanelVertex {
    pub fn new<P: Into<RawPosition>>(position: P, listen_offset: ListenOffset) -> Self {
        Self {
            position: position.into(),
            listen_offset,
        }
    }
}
pub(crate) fn generate_panel_corner(
    current: f32,
    current_corner: Position<DeviceContext>,
    delta: f32,
    x_offset: bool,
    y_offset: bool,
    scale_factor: f64,
) -> Vec<PanelVertex> {
    let mut tris = Vec::new();
    let listen_offset = ListenOffset::new(
        ListenOffset::from_bool(x_offset),
        ListenOffset::from_bool(y_offset),
    );
    let mut current_angle = current;
    let mut last =
        current_corner + position_from_angle(current_angle, Panel::CORNER_DEPTH, scale_factor);
    let mut interpolator = Interpolator::new(FRAC_PI_2);
    let mut extraction = interpolator.extract(delta);
    current_angle += extraction.0;
    let point =
        current_corner + position_from_angle(current_angle, Panel::CORNER_DEPTH, scale_factor);
    tris.push(current_corner);
    tris.push(last);
    tris.push(point);
    last = point;
    while !extraction.1 {
        extraction = interpolator.extract(delta);
        current_angle += extraction.0;
        let point =
            current_corner + position_from_angle(current_angle, Panel::CORNER_DEPTH, scale_factor);
        tris.push(current_corner);
        tris.push(last);
        tris.push(point);
        last = point;
    }
    let mut corner_tris = tris
        .iter()
        .map(|vertex| -> PanelVertex { PanelVertex::new(vertex.as_raw(), listen_offset) })
        .collect::<Vec<PanelVertex>>();
    let mut bar_tris = Vec::new();
    let near_inner = PanelVertex::new(current_corner.as_raw(), listen_offset);
    let near_outer = PanelVertex::new(last.as_raw(), listen_offset);
    let mut far_inner = PanelVertex::new(current_corner.as_raw(), listen_offset);
    let mut far_outer = PanelVertex::new(last.as_raw(), listen_offset);
    if x_offset && y_offset {
        far_inner.listen_offset.listen_y = ListenOffset::from_bool(false);
        far_outer.listen_offset.listen_y = ListenOffset::from_bool(false);
    } else if x_offset && !y_offset {
        far_inner.listen_offset.listen_x = ListenOffset::from_bool(false);
        far_outer.listen_offset.listen_x = ListenOffset::from_bool(false);
    } else if !x_offset && y_offset {
        far_inner.listen_offset.listen_x = ListenOffset::from_bool(true);
        far_outer.listen_offset.listen_x = ListenOffset::from_bool(true);
    } else {
        far_inner.listen_offset.listen_y = ListenOffset::from_bool(true);
        far_outer.listen_offset.listen_y = ListenOffset::from_bool(true);
    }
    bar_tris.push(near_inner);
    bar_tris.push(near_outer);
    bar_tris.push(far_outer);
    bar_tris.push(far_outer);
    bar_tris.push(far_inner);
    bar_tris.push(near_inner);
    corner_tris.extend(bar_tris);
    corner_tris
}
pub(crate) fn position_from_angle(
    angle: f32,
    radius: f32,
    scale_factor: f64,
) -> Position<DeviceContext> {
    Position::<InterfaceContext>::from((angle.cos() * radius, -angle.sin() * radius))
        .to_device(scale_factor)
}
pub(crate) fn generate_panel_mesh(corner_precision: u32, scale_factor: f64) -> Vec<PanelVertex> {
    let delta = 1f32 / corner_precision as f32;
    let mut mesh = Vec::new();
    let center = Position::<InterfaceContext>::from((Panel::CORNER_DEPTH, Panel::CORNER_DEPTH))
        .to_device(scale_factor);
    mesh.extend(generate_panel_corner(
        FRAC_PI_2,
        center,
        delta,
        false,
        false,
        scale_factor,
    ));
    mesh.extend(generate_panel_corner(
        PI,
        center,
        delta,
        false,
        true,
        scale_factor,
    ));
    mesh.extend(generate_panel_corner(
        3.0 * FRAC_PI_2,
        center,
        delta,
        true,
        true,
        scale_factor,
    ));
    mesh.extend(generate_panel_corner(
        0f32,
        center,
        delta,
        true,
        false,
        scale_factor,
    ));
    mesh.push(PanelVertex::new(
        center.as_raw(),
        ListenOffset::new(
            ListenOffset::from_bool(false),
            ListenOffset::from_bool(false),
        ),
    ));
    mesh.push(PanelVertex::new(
        center.as_raw(),
        ListenOffset::new(
            ListenOffset::from_bool(false),
            ListenOffset::from_bool(true),
        ),
    ));
    mesh.push(PanelVertex::new(
        center.as_raw(),
        ListenOffset::new(
            ListenOffset::from_bool(true),
            ListenOffset::from_bool(false),
        ),
    ));
    mesh.push(PanelVertex::new(
        center.as_raw(),
        ListenOffset::new(
            ListenOffset::from_bool(true),
            ListenOffset::from_bool(false),
        ),
    ));
    mesh.push(PanelVertex::new(
        center.as_raw(),
        ListenOffset::new(
            ListenOffset::from_bool(false),
            ListenOffset::from_bool(true),
        ),
    ));
    mesh.push(PanelVertex::new(
        center.as_raw(),
        ListenOffset::new(ListenOffset::from_bool(true), ListenOffset::from_bool(true)),
    ));
    mesh
}
pub(crate) fn vertex_buffer(gfx_surface: &GfxSurface, mesh: Vec<PanelVertex>) -> wgpu::Buffer {
    gfx_surface
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("panel vertex buffer"),
            contents: bytemuck::cast_slice(&mesh),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        })
}
pub(crate) fn generate_border_corner(
    current: f32,
    current_corner: Position<DeviceContext>,
    delta: f32,
    x_offset: bool,
    y_offset: bool,
    scale_factor: f64,
) -> Vec<PanelVertex> {
    let mut tris = Vec::new();
    let listen_offset = ListenOffset::new(
        ListenOffset::from_bool(x_offset),
        ListenOffset::from_bool(y_offset),
    );
    let line_width_offset = Panel::CORNER_DEPTH - Panel::LINE_WIDTH;
    let mut current_angle = current;
    let mut outer_bottom =
        current_corner + position_from_angle(current_angle, Panel::CORNER_DEPTH, scale_factor);
    let mut inner_bottom =
        current_corner + position_from_angle(current_angle, line_width_offset, scale_factor);
    let mut interpolator = Interpolator::new(FRAC_PI_2);
    let mut extraction = interpolator.extract(delta);
    current_angle += extraction.0;
    let mut outer_top =
        current_corner + position_from_angle(current_angle, Panel::CORNER_DEPTH, scale_factor);
    let mut inner_top =
        current_corner + position_from_angle(current_angle, line_width_offset, scale_factor);
    tris.extend([inner_top, inner_bottom, outer_top]);
    tris.extend([inner_bottom, outer_bottom, outer_top]);
    inner_bottom = inner_top;
    outer_bottom = outer_top;
    while !extraction.1 {
        extraction = interpolator.extract(delta);
        current_angle += extraction.0;
        outer_top =
            current_corner + position_from_angle(current_angle, Panel::CORNER_DEPTH, scale_factor);
        inner_top =
            current_corner + position_from_angle(current_angle, line_width_offset, scale_factor);
        tris.extend([inner_top, inner_bottom, outer_top]);
        tris.extend([inner_bottom, outer_bottom, outer_top]);
        inner_bottom = inner_top;
        outer_bottom = outer_top;
    }
    let mut corner_tris = tris
        .iter()
        .map(|vertex| -> PanelVertex { PanelVertex::new(vertex.as_raw(), listen_offset) })
        .collect::<Vec<PanelVertex>>();
    let mut bar_tris = Vec::new();
    let near_inner = PanelVertex::new(inner_top.as_raw(), listen_offset);
    let near_outer = PanelVertex::new(outer_top.as_raw(), listen_offset);
    let mut far_inner = PanelVertex::new(inner_top.as_raw(), listen_offset);
    let mut far_outer = PanelVertex::new(outer_top.as_raw(), listen_offset);
    if x_offset && y_offset {
        far_inner.listen_offset.listen_y = ListenOffset::from_bool(false);
        far_outer.listen_offset.listen_y = ListenOffset::from_bool(false);
    } else if x_offset && !y_offset {
        far_inner.listen_offset.listen_x = ListenOffset::from_bool(false);
        far_outer.listen_offset.listen_x = ListenOffset::from_bool(false);
    } else if !x_offset && y_offset {
        far_inner.listen_offset.listen_x = ListenOffset::from_bool(true);
        far_outer.listen_offset.listen_x = ListenOffset::from_bool(true);
    } else {
        far_inner.listen_offset.listen_y = ListenOffset::from_bool(true);
        far_outer.listen_offset.listen_y = ListenOffset::from_bool(true);
    }
    bar_tris.push(near_inner);
    bar_tris.push(near_outer);
    bar_tris.push(far_outer);
    bar_tris.push(far_outer);
    bar_tris.push(far_inner);
    bar_tris.push(near_inner);
    corner_tris.extend(bar_tris);
    corner_tris
}
pub(crate) fn generate_border_mesh(corner_precision: u32, scale_factor: f64) -> Vec<PanelVertex> {
    let delta = 1f32 / corner_precision as f32;
    let mut mesh = Vec::new();
    let center = Position::<InterfaceContext>::from((Panel::CORNER_DEPTH, Panel::CORNER_DEPTH))
        .to_device(scale_factor);
    mesh.extend(generate_border_corner(
        FRAC_PI_2,
        center,
        delta,
        false,
        false,
        scale_factor,
    ));
    mesh.extend(generate_border_corner(
        PI,
        center,
        delta,
        false,
        true,
        scale_factor,
    ));
    mesh.extend(generate_border_corner(
        3.0 * FRAC_PI_2,
        center,
        delta,
        true,
        true,
        scale_factor,
    ));
    mesh.extend(generate_border_corner(
        0f32,
        center,
        delta,
        true,
        false,
        scale_factor,
    ));
    mesh
}
