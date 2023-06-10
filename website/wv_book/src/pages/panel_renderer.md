# Panel Renderer

The `PanelRenderer` makes a visual container at a location specified by the [`Grid`](grid.md) using a `Panel`.
Given a `ResponsiveGridView` this bundle will spawn a rounded corner panel that colors the background with
`PanelColor`. This can be augmented with `BorderColor` and `PanelType::BorderedPanel` to draw a colored 
rounded line around the perimeter.

### Panel Type
Three `PanelType`s exist.
```rust
pub enum PanelType {
    Panel,
    Border,
    BorderedPanel,
}
```
This is done because the same logic for rendering a `Border` coexists with 
the systems for `Panel`. To specify a `Border` only use `PanelType::Border`. 
Specifying a `BorderColor` when only using `PanelType::Panel` will ignore the 
`BorderColor` component.

### Padding

The `CornerDepth` of a `Panel` is set to `5px` to give the rounded corner room
to bend without interfering with the content there. The `Panel` is adjusted `5px` to 
the top/left to adjust for this. 