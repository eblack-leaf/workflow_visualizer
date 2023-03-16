struct Viewport {
    view_matrix: mat4x4<f32>,
};
@group(0)
@binding(0)
var<uniform> viewport: Viewport;
@group(0)
@binding(1)
var<uniform> viewport_offset: vec4<f32>;
struct VertexInput {
    @location(0) vertex_data: vec4<f32>,
    @location(1) position: vec2<f32>,
    @location(2) content_area: vec2<f32>,
    @location(3) layer: f32,
    @location(4) color: vec4<f32>,
    @location(5) null_bit: u32,
};
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
};
@vertex
fn vertex_entry(vertex_input: VertexInput) -> VertexOutput {
    let nulled = bool(vertex_input.null_bit) == true;
    let null_mult = f32(!nulled);
    let offset_pos = vertex_input.position - viewport_offset.xy;
    let content_scaled = offset_pos + vertex_input.vertex_data.xy + vertex_input.content_area * vertex_input.vertex_data.zw;
    let vertex_out_pos = vec4<f32>(content_scaled, vertex_input.layer, 1.0);
    return VertexOutput(viewport.view_matrix * vertex_out_pos * null_mult, vertex_input.color);
}
@fragment
fn fragment_entry(vertex_output: VertexOutput) -> @location(0) vec4<f32> {
// try noise to give some texture matte
    return vertex_output.color;
}
