# Orientation

`Orientation` is set on window resizing to be read if needing to respond to changes in
orientation. Available options are 
```rust
pub enum Orientation {
    Portrait(f32),
    Landscape(f32),
}
```
They hold the aspect ratio of each option for reference. 