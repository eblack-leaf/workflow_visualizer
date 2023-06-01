# Gfx
This holds the wgpu `Surface` | `Device` | `Queue` | `Config`. These parts obtain a drawable
surface, a gpu device to use for acceleration, a queue to submit commands to this gpu, and the
configuration of the surface (formats / size). This is needed to power all the render pipelines
and present to the window created during run. 

### GfxOptions

When creating a `Visualizer`, you will need to pass it `GfxOptions`. This can be used to configure how
your application uses the `Gfx` stack and what environment is needed. Desktop platforms generally can run
`GfxOptions::native_defaults()` without concern. Mobile/Web platforms might want to downgrade the limits using
`GfxOptions::limited_environment()` which will lower certain aspects commonly not implemented on these platforms.

Your main interaction with this struct will be extending the visualizer with custom pipelines. 
```rust
fn system(gfx: Res<GfxSurface>, config: Res<GfxSurfaceConfiguration>, msaa: Res<MsaaRenderAdapter>) {
    // ... use parts
    // gfx.device/surface/queue
    // config.config.width/height
    // msaa.requested(); // the requested level of sampling/anti-aliasing from
}
```
The `MsaaRenderAdapter` can be used to request multi-sampling and anti-aliasing for your renderers e.g.
`GfxOptions::native_defaults().with_msaa(4)`.

### Learn More

For more information check out the amazing work being done at [`wgpu.rs`](https://wgpu.rs/). 