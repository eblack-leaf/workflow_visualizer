# Coordinates

To position things, the struct `Coordinates` helps by providing
- `Position` the x/y of the entity
- `Area` the width/height of the entity
- `Layer` the z-value of the entity for ordering depth
This is all wrapped in a 
```rust
pub struct Coordinate<Context: CoordContext> {
    #[bundle]
    pub section: Section<Context>,
    pub layer: Layer,
}
```
### Parts
The `Coordinate` struct is a composition of another bundle `Section` and a `Layer`. A `Section` is
a bundle of `Position` and `Area`. This spawns all the components separately for orthogonality in 
designing systems to utilize these parts.


A `Coordinate` uses a `CoordinateContext` to differentiate different
types of coordinate values. There are 3 contexts provided by the visualizer.
```rust
impl CoordinateContext for DeviceContext {}
impl CoordinateContext for InterfaceContext {}
impl CoordinateContext for NumericalContext {}
```
### Scale | Dpi
All UI elements should be assigned coordinates within the [`Grid`](grid.md), but for 
understanding how coordinates work, UI coordinates use `InterfaceContext` to display correctly on devices
that require scaling to present correctly. Mobile devices have high resolution screens yet are
a fraction of the size of desktop monitors. Drawing a shape at 50x50 on desktop would be very small
on a device with higher dots per inch (dpi). `ScaleFactor` holds a f64 to be used to multiply to get the
needed size of an element in `DeviceContext`. It is used to divide `DeviceContext` coordinates to get to
`InterfaceContext`. Developers should design for the middle ground of 1920x1080 which is considered a
scale factor of `1`. The same `InterfaceContext` coordinates will be correctly scaled to fit on different
sized devices. The `Viewport`'s interface section will be smaller on higher dpi devices however, so using raw 
coordinates might be off-screen when running the same layout on desktop/mobile. To account for this
a [`Grid`](grid.md) is established and should be the primary way to instruct UI elements where to be.
#### DeviceContext
This context is for coordinates in physical space such as hardware dimensions for the window.
The `desktop_dimensions` that are passed to `Runner::new().with_desktop_dimensions(...)` is an 
example of `DeviceContext`.
#### InterfaceContext 
This context is for coordinates in logical space such as UI positioning. This is needed to 
account for scale factor of the device being run on. `ScaleFactor` holds a reference to 
the platforms info to be used in conversion between contexts.
```rust
fn system(scale_factor: Res<ScaleFactor>) {
    // ...
    let interface_context_pos = device_context_pos.to_interface(scale_factor.factor);
}
```
#### NumericalContext
This context is agnostic to device/logical and is used for representing 
numbers without associating screen positioning semantics. Useful for inheriting the
methods of `Coordinate` without it mixing with your interface positioning.
```rust 
let non_ui_coord = Coordinate::<NumericalContext>::new(...);
```

