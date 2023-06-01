# Viewport
The `Viewport` is central to all rendering as it integral to correctly positioning
elements. Here is the definition.

```rust
pub struct Viewport {
    pub(crate) cpu: CpuViewport,
    pub(crate) gpu: GpuViewport,
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub(crate) uniform: Uniform<GpuViewport>,
    pub(crate) depth_texture: wgpu::Texture,
    pub(crate) depth_format: wgpu::TextureFormat,
    pub(crate) offset: ViewportOffset,
    pub(crate) offset_uniform: Uniform<ViewportOffset>,
}
```

This holds the data for an 
[`orthographic`](https://www.scratchapixel.com/lessons/3d-basic-rendering/perspective-and-orthographic-projection-matrix/orthographic-projection-matrix.html)
matrix that converts coordinates to NDC coordinates which are limited to -1/1, from screen coordinates such
as the elements `Coordinate`. It takes the current window size and creates a logical version (Cpu) and a 
uniform buffer on the `Gpu`. This needs to be used via the `pub` fields `bind_group` and `bind_group_layout` 
in shaders to multiply against to get the correct normalized position.
```wgsl
struct Viewport {
    view_matrix: mat4x4<f32>,
};
@group(0)
@binding(0)
var<uniform> viewport: Viewport;
@group(0)
@binding(1)
var<uniform> viewport_offset: vec4<f32>;
// ... {
    var offset_coordinates = vec4<f32>(coordinates.rg - viewport_offset.rg, coordinates.ba);
    let ndc = vec4<f32>(viewport.view_matrix * offset_coordinates);
// ... }
```
The `viewport` is the matrix which needs to be multiplied by your coordinates to return NDC coordinates.
This is what is expected by the shader vertex output.

The `viewport_offset` is the current position of the viewport used to scroll where the UI is looking 
at. This is set using `ViewportHandle`. You must first offset your elements x/y before multiplying.
```rust
fn system(handle: ResMut<ViewportHandle>) {
    handle.position_adjust(...);// to change offset
    let section: Section<InterfaceContext> = handle.section();// for using bounds of Viewport 
}
```

This is in two parts to allow systems running things with the rendering tool `Viewport` do not
interfere with the logical handle `ViewportHandle` and thus have two different uniforms. This is also
performant for updating position repeatedly as the UI scrolls because the matrix data does not need to be 
changed as the `Viewport` is not changing size and thus less sent over to gpu. 

### Depth

The `Viewport` also holds the depth buffer as it is correlated in size to the `Viewport` dimensions
and must be recreated when resized. The near layer is `Layer::new(0)` and the far is `Layer::new(100)`.
All layers should be within this bound to not be clipped by the `Viewport` during culling by the gpu.

#### Usage in Renderers
Include the bind_group_layout
```rust
let layout_descriptor = wgpu::PipelineLayoutDescriptor {
        label: Some("pipeline layout descriptor"),
        bind_group_layouts: &[
            &viewport.bind_group_layout,
            // ...
        ],
        push_constant_ranges: &[],
    };
```
Specify the depth format
```rust
let depth_stencil_state = Some(wgpu::DepthStencilState {
        format: viewport.depth_format(),
        // ...
    });
```
Set the `bind_group` in your `Render::render` fn
```rust
render_pass_handle
            .0
            .set_bind_group(0, &viewport.bind_group, &[]);
```