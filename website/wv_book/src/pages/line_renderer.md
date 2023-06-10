# Line Renderer

Lines are an extension of the [`Path`](path.md) module that actually establishes the
positions for the line. Additionally, it takes a `Layer` and `Color` for style guidelines.

```rust
Line::new(
ResponsivePathView::all_same(vec![
    (1.near(), 1.far()).into(),
    (4.far(), 1.far()).into(),
]),
1,
Color::MEDIUM_GREEN,
)
```

This will draw a line from the `GridLocation` `x`/`y` of `1.near()`/`1.far()` to `4.far()`/`1.far()`.