use crate::icon::{write_mesh, ColorHooks, IconVertex};
use crate::DeviceContext;

#[cfg(test)]
#[test]
fn generate_panel_mesh() {
    use crate::Position;
    use crate::{ColorHooks, DeviceView, IconVertex};
    use std::f32::consts::{FRAC_PI_2, PI};
    let mut mesh = Vec::<IconVertex>::new();
    let mut corners = Vec::new();
    let corner_precision = 444.0;
    let radius = 0.050f32;
    let mut tris = Vec::new();
    {
        let corner_center = Position::<DeviceContext>::new(1.0 - radius, radius);
        corners.push(corner_center);
        let total = FRAC_PI_2;
        let interval = total / corner_precision;
        let end = FRAC_PI_2;
        let mut current = 0f32;
        let mut last = corner_center
            + Position::<DeviceContext>::from((current.cos() * radius, -current.sin() * radius));
        current += interval;
        while current < end {
            let x = current.cos() * radius;
            let y = current.sin() * radius;
            let point = corner_center + Position::<DeviceContext>::from((x, -y));
            let tri = vec![corner_center, last, point];
            tris.push(tri);
            last = point;
            current += interval;
        }
        let bar_bottom = corner_center;
        let bar_top = Position::<DeviceContext>::from((bar_bottom.x, bar_bottom.y - radius));
        let other_top = Position::from((radius, 0.0));
        let other_bottom = Position::from((other_top.x, other_top.y + radius));
        let tri = vec![bar_bottom, bar_top, other_top];
        tris.push(tri);
        let tri = vec![other_top, other_bottom, bar_bottom];
        tris.push(tri);
    }
    {
        let corner_center = Position::<DeviceContext>::from((radius, radius));
        corners.push(corner_center);
        let mut current = FRAC_PI_2;
        let total = FRAC_PI_2;
        let end = PI;
        let mut last = corner_center
            + Position::<DeviceContext>::from((current.cos() * radius, -current.sin() * radius));
        let interval = total / corner_precision;
        current += interval;
        while current < end {
            let x = current.cos() * radius;
            let y = current.sin() * radius;
            let point = corner_center + Position::<DeviceContext>::from((x, -y));
            let tri = vec![corner_center, last, point];
            tris.push(tri);
            last = point;
            current += interval;
        }
        let bar_bottom = corner_center;
        let bar_top = Position::<DeviceContext>::from((bar_bottom.x - radius, bar_bottom.y));
        let other_top = Position::from((0.0, 1.0 - radius));
        let other_bottom = Position::from((other_top.x + radius, other_top.y));
        let tri = vec![bar_bottom, bar_top, other_top];
        tris.push(tri);
        let tri = vec![other_top, other_bottom, bar_bottom];
        tris.push(tri);
    }
    {
        let corner_center = Position::<DeviceContext>::from((radius, 1.0 - radius));
        corners.push(corner_center);
        let mut current = PI;
        let total = FRAC_PI_2;
        let end = 3.0 * FRAC_PI_2;
        let mut last = corner_center
            + Position::<DeviceContext>::from((current.cos() * radius, -current.sin() * radius));
        let interval = total / corner_precision;
        current += interval;
        while current < end {
            let x = current.cos() * radius;
            let y = current.sin() * radius;
            let point = corner_center + Position::<DeviceContext>::from((x, -y));
            let tri = vec![corner_center, last, point];
            tris.push(tri);
            last = point;
            current += interval;
        }
        let bar_bottom = corner_center;
        let bar_top = Position::<DeviceContext>::from((bar_bottom.x, bar_bottom.y + radius));
        let other_top = Position::from((1.0 - radius, 1.0));
        let other_bottom = Position::from((other_top.x, other_top.y - radius));
        let tri = vec![bar_bottom, bar_top, other_top];
        tris.push(tri);
        let tri = vec![other_top, other_bottom, bar_bottom];
        tris.push(tri);
    }
    {
        let corner_center = Position::<DeviceContext>::from((1.0 - radius, 1.0 - radius));
        corners.push(corner_center);
        let mut current = 3.0 * FRAC_PI_2;
        let total = FRAC_PI_2;
        let end = 2.0 * PI;
        let mut last = corner_center
            + Position::<DeviceContext>::from((current.cos() * radius, -current.sin() * radius));
        let interval = total / corner_precision;
        current += interval;
        while current < end {
            let x = current.cos() * radius;
            let y = current.sin() * radius;
            let point = corner_center + Position::<DeviceContext>::from((x, -y));
            let tri = vec![corner_center, last, point];
            tris.push(tri);
            last = point;
            current += interval;
        }
        let bar_bottom = corner_center;
        let bar_top = Position::<DeviceContext>::from((bar_bottom.x + radius, bar_bottom.y));
        let other_top = Position::from((1.0, radius));
        let other_bottom = Position::from((other_top.x - radius, other_top.y));
        let tri = vec![bar_bottom, bar_top, other_top];
        tris.push(tri);
        let tri = vec![other_top, other_bottom, bar_bottom];
        tris.push(tri);
    }
    let inner_mesh_one = vec![corners[0], corners[1], corners[2]];
    tris.push(inner_mesh_one);
    let inner_mesh_two = vec![corners[0], corners[2], corners[3]];
    tris.push(inner_mesh_two);
    for tri in tris {
        for vertex in tri {
            mesh.push(IconVertex::new(
                vertex.to_gpu(),
                ColorHooks::new(ColorHooks::POSITIVE_SPACE, ColorHooks::HOOKABLE),
            ));
        }
    }
    write_mesh(
        &mesh,
        "/home/omi-voshuli/Desktop/note-ifications/mise_en_place/src/icon/icons/panel.icon_mesh",
    );
}

#[cfg(test)]
#[test]
fn generate_cursor_mesh() {
    use crate::Position;
    let mut mesh = Vec::<IconVertex>::new();
    mesh.push(IconVertex::new(
        Position::<DeviceContext>::new(0.0, 0.0).to_gpu(),
        ColorHooks::new(ColorHooks::POSITIVE_SPACE, ColorHooks::HOOKABLE),
    ));
    mesh.push(IconVertex::new(
        Position::<DeviceContext>::new(0.0, 1.0).to_gpu(),
        ColorHooks::new(ColorHooks::POSITIVE_SPACE, ColorHooks::HOOKABLE),
    ));
    mesh.push(IconVertex::new(
        Position::<DeviceContext>::new(1.0, 0.0).to_gpu(),
        ColorHooks::new(ColorHooks::POSITIVE_SPACE, ColorHooks::HOOKABLE),
    ));
    mesh.push(IconVertex::new(
        Position::<DeviceContext>::new(1.0, 0.0).to_gpu(),
        ColorHooks::new(ColorHooks::POSITIVE_SPACE, ColorHooks::HOOKABLE),
    ));
    mesh.push(IconVertex::new(
        Position::<DeviceContext>::new(0.0, 1.0).to_gpu(),
        ColorHooks::new(ColorHooks::POSITIVE_SPACE, ColorHooks::HOOKABLE),
    ));
    mesh.push(IconVertex::new(
        Position::<DeviceContext>::new(1.0, 1.0).to_gpu(),
        ColorHooks::new(ColorHooks::POSITIVE_SPACE, ColorHooks::HOOKABLE),
    ));
    crate::write_mesh(
        &mesh,
        "/home/omi-voshuli/Desktop/note-ifications/mise_en_place/src/icon/icons/cursor.icon_mesh",
    );
}
