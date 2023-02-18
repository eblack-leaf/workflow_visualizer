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
    @location(0) vertex_position: vec2<f32>,
    @location(1) position: vec2<f32>,
    @location(2) area: vec2<f32>,
    @location(3) depth: f32,
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
    var coordinates = vec4<f32>(
        vertex_input.position.x + vertex_input.vertex_position.x * vertex_input.area.x,
        vertex_input.position.y + vertex_input.vertex_position.y * vertex_input.area.y,
        vertex_input.depth,
        1.0);
    var offset_coordinates = vec4<f32>(coordinates.rg - viewport_offset.rg, coordinates.ba);
    return VertexOutput(viewport.view_matrix * offset_coordinates * null_mult, vertex_input.color);
}
@fragment
fn fragment_entry(vertex_output: VertexOutput) -> @location(0) vec4<f32> {
    return vertex_output.color;
}