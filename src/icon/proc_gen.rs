#[cfg(test)]
#[test]
fn generate_cursor_mesh() {
    use crate::ColorHooks;
    use crate::DeviceContext;
    use crate::IconVertex;
    use crate::Position;
    let mut mesh = Vec::<IconVertex>::new();
    mesh.push(IconVertex::new(
        Position::<DeviceContext>::new(0.0, 0.0).as_raw(),
        ColorHooks::new(ColorHooks::POSITIVE_SPACE, ColorHooks::HOOKABLE),
    ));
    mesh.push(IconVertex::new(
        Position::<DeviceContext>::new(0.0, 1.0).as_raw(),
        ColorHooks::new(ColorHooks::POSITIVE_SPACE, ColorHooks::HOOKABLE),
    ));
    mesh.push(IconVertex::new(
        Position::<DeviceContext>::new(1.0, 0.0).as_raw(),
        ColorHooks::new(ColorHooks::POSITIVE_SPACE, ColorHooks::HOOKABLE),
    ));
    mesh.push(IconVertex::new(
        Position::<DeviceContext>::new(1.0, 0.0).as_raw(),
        ColorHooks::new(ColorHooks::POSITIVE_SPACE, ColorHooks::HOOKABLE),
    ));
    mesh.push(IconVertex::new(
        Position::<DeviceContext>::new(0.0, 1.0).as_raw(),
        ColorHooks::new(ColorHooks::POSITIVE_SPACE, ColorHooks::HOOKABLE),
    ));
    mesh.push(IconVertex::new(
        Position::<DeviceContext>::new(1.0, 1.0).as_raw(),
        ColorHooks::new(ColorHooks::POSITIVE_SPACE, ColorHooks::HOOKABLE),
    ));
    crate::write_mesh(
        &mesh,
        "/home/omi-voshuli/Desktop/note-ifications/mise_en_place/src/icon/icons/cursor.icon_mesh",
    );
}
