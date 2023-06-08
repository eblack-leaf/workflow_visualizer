# Grid

The `Grid` is responsible for placing elements consistently across different screen
sizes and scales. Elements that employ the use of `ResponsiveGridView` will have the
views defined there configured to an adaptive grid of 8px `Marker`s that mark points
for assigning `Position`s. The `Viewport` area is divided by these `Marker`s and grouped
to become columns/rows. The `ColumnConfig` stores a `base` `MarkerGrouping` which denotes
how many `Marker`s are in a column. The `RowConfig` takes after the size of the column
`base` for consistent alignment. The `ColumnConfig` also uses another `MarkerGrouping`
for an `extension` which adds space evenly to all columns if the `Viewport` width is not
covered by the `base` column size. Between exist `Gutter`s that help space content consistently.
The `GutterConfig` starts small and then extends the gutter size when more screen space is
available.

### Span

There are 3 `Span`s to the grid, each of which contains different `Grid` configuration.

```rust
pub enum HorizontalSpan {
    Four,
    Eight,
    Twelve,
}
```

The name denotes the number of logical columns available at that size.
The `Viewport` size determines the `Span`.

- From ~ - 720px the `Grid` uses `HorizontalSpan::Four` (Mobile)
- From 721px - 1168px the `Grid` uses `HorizontalSpan::Eight` (Tablet)
- From 1169px - ~ the `Grid` uses `HorizontalSpan::Twelve` (Desktop)

This allows predetermined logical spacers to guide placement across a
valid range of device sizes that need adaptive positioning. Knowing the exact
column count instead of growing with variable columns as needed keeps the placement consistent.
This does require that a view be specified for each `HorizontalSpan` however.

### Usage

To use the grid, add 'ResponsiveGridView' as a component to an entity.

```rust
visualizer.add_entities(vec![Request::new((
    ResponsiveGridView::all_same(((1.near(), 4.far()), (1.near(), 1.far().offset(-4)))),
    // ... more components
))]);
```

This example sets the same view for each `HorizontalSpan` with `::all_same(view)`.
The notation is {`GridMarker`}.{`GridMarkerBias`}(){.offset(`RawMarker`)}.
This signifies that the `1` should be the `GridMarker` or the index of the logical grouping
of `RawMarker`s is the first column. The `Column` has a `.near()` and a `.far()` bias.
This can be used to latch to either side of the `Column`. An `.offset(...)` is used
for more fine-grained positioning where just a `Column` and a `Bias` does not suffice.
The `.offset(...)` adds a relative offset of `RawMarker` using the 8px grid points directly
instead of a logical column/row grouping index to add to the initial marker placing.
The points are grouped in two pairs. Horizontal begin/end and Vertical begin/end.
This constitutes a `GridRange` when specified for `horizontal` and `vertical` it makes
a `GridView` that can be calculated to obtain a `Position` | `Area`.

This would be all that is needed except that the `Grid` has different
column numbers at different `Span`s. Setting mobile first guidelines using the
`HorizontalSpan::Four` and `::all_same(...)` is a good starting place and will work
on larger `Span`s as four columns exist within 8/12. To maximize benefits of the `Grid`
an `ResponsiveGridView::explicit(four, eight, twelve)` which defines elements using
the entire available space should be used. If three layouts are not needed, a
`ResponsiveGridView` can be used with `.with_span_eight(...)` to only specify an
extra layout for the `HorizontalSpan::Eight`.

### Helpers

`Grid.calc_section(view)` can be used to get the `Section<InterfaceContext>` for the
current `Span`. Similarly, `Grid.calc_horizontal_location(location)` and
`Grid.calc_vertical_location(location)` can obtain the `RawMarker` that corresponds
to the given location.

### ResponsiveView<T>

This type is the base for `ResponsiveGridView`

```rust
pub type ResponsiveGridView = ResponsiveView<GridView>;
```

Which maps a `GridView` to each `HorizontalSpan`.
