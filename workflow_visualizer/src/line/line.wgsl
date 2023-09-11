struct Viewport {
    view_matrix: mat4x4<f32>,
};
@group(0)
@binding(0)
var<uniform> viewport: Viewport;
@group(0)
@binding(1)
var<uniform> viewport_offset: vec4<f32>;
@group(1)
@binding(0)
var<uniform> color: vec4<f32>;
@group(1)
@binding(1)
var<uniform> layer_and_hooks: vec4<f32>;
struct VertexInput {
    @location(0) position: vec2<f32>,
};
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(1) color: vec4<f32>,
}
@vertex
fn vertex_entry(vertex_input: VertexInput) -> VertexOutput {
    let offset_coordinates = vec4<f32>(vertex_input.position.xy - viewport_offset.xy, layer_and_hooks.r, 1.0);
    return VertexOutput(viewport.view_matrix * offset_coordinates, color);
}
@fragment
fn fragment_entry(fragment_input: VertexOutput) -> @location(0) vec4<f32> {
    if (fragment_input.color.a <= 0.0) { discard; }
    return fragment_input.color;
}