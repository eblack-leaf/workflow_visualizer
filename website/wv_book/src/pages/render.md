# Render

Rendering is the main purpose of the `Visualizer`. [`Gfx`](gfx.md) module points to information
on how this lib uses [wgpu.rs](https://wgpu.rs) to achieve this goal. The core part of rendering is
the `wgpu::RenderPass` which allows recording of commands to the gpu available. This loads the memory
for the surface and rasterizes the draw calls. Presentation of this surface works in FIFO mode (swapping
surface textures is a chain to not interrupt device rendering) and constrains itself to a single `RenderPass`
as loading memory to do a second render pass is very costly on (especially on mobile devices). This centralizes
where rendering happens and creates an efficient solution for presenting to the device.

### Render Trait

```rust
pub trait Render {
    fn phase() -> RenderPhase;
    fn render<'a>(&'a self, render_pass_handle: &mut RenderPassHandle<'a>, viewport: &'a Viewport);
}
```

The `render` fn allows access to the `RenderPass` via a handle and the [`Viewport`](viewport.md) to
integrate the `bind_group`/`layout` into shaders. Access to `&self` is available to hold any
data needed to render such as `wgpu::Buffer`s or `wgpu::Texture`s.

`fn phase() -> RenderPhase` is for signaling what phase this `render` should take place in.
`RenderPhase::Opaque` fns are run first as no blending is present.
`RenderPhase::Alpha(priority)` is used for renderers that wish to use alpha-blending and 
happens after opaque renderers to blend correctly with the colors underneath in the depth buffer.