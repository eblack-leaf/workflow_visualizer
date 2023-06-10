# Path

Pathing is available for guiding animations or instructing where to draw lines.
Each path is made up of `PathViewPoints` which uses the [`Grid`](grid.md) to
align the actual `Path` points which are `Position<InterfaceContext>`s. This is
held in a `ResponsivePathView` which maps `ResponsiveView<T>` to `PathView`. A
`PathView` is a collection of `PathViewPoints` that get calculated to the
specific `Position<InterfaceContext>` depending on the grid `Span`.

### Usages

The `Path` that gets calculated can be a base for positions to
move to in animations or as the points of a line strip to render
lines to the UI.

```rust
ResponsivePathView::all_same(vec![
            (1.near(), 1.far()).into(),
            (4.far(), 1.far()).into(),
            // ... more points in path
        ])
```