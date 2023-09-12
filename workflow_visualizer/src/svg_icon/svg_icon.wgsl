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
    @location(0) vertex_data: vec2<f32>,
    @location(1) position: vec2<f32>,
    @location(2) area: vec2<f32>,
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
    let position = vertex_input.position + vertex_input.vertex_data * vertex_input.area;
    let offset_pos = position - viewport_offset.xy;
    let vertex_out_pos = vec4<f32>(offset_pos, vertex_input.layer, 1.0);
    return VertexOutput(viewport.view_matrix * vertex_out_pos * null_mult, vertex_input.color);
}
@fragment
fn fragment_entry(vertex_output: VertexOutput) -> @location(0) vec4<f32> {
    if (vertex_output.color.a <= 0.0) { discard; }
    return vertex_output.color;
}