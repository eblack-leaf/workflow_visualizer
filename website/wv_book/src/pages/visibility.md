# Visibility

The `Visibility` struct handles the notion of being on-screen and by what amount with `VisibleSection`.
To enable this in an entity add the `EnableVisibility` bundle which hooks it
into the `calc_visibility` system.
This is run twice every frame; once after initialization, and once after processing completes.

```rust
let bundle = (..., EnableVisibility::default());
// ...
if visibility.visible() {
    // ... do something because visible
}
```

When an entity is partially off-screen it can be helpful to read from the `VisibleSection`.
```rust
if let Some(section) = visible_section.section() {
    // ... react according to actual in-screen pos/area
}
```