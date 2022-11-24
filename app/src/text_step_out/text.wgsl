struct Viewport {
    view_matrix: mat4x4<f32>,
};
@group(0)
@binding(0)
var<uniform> viewport: Viewport;
struct VertexInput {
    @location(0) vertex_position: vec2<f32>,
    @location(1) instance_position: vec2<f32>,
    @location(2) instance_area: vec2<f32>,
    @location(3) instance_depth: f32,
    @location(4) instance_color: vec4<f32>,
    @location(5) instance_rasterization_descriptor: vec3<u32>,

};
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) instance_rasterization_descriptor: vec3<u32>,
    @location(1) color: vec4<f32>,
    @location(2) instance_position: vec2<f32>,
};
struct FragmentInput {
    @builtin(position) position: vec4<f32>,
    @location(0) instance_rasterization_descriptor: vec3<u32>,
    @location(1) color: vec4<f32>,
    @location(2) instance_position: vec2<f32>,
};
@vertex
fn vertex_entry(
    vertex_input: VertexInput,
) -> VertexOutput {
    let coordinates = vec4<f32>(
        vertex_input.instance_position[0] + (vertex_input.vertex_position[0] * vertex_input.instance_area[0]),
        vertex_input.instance_position[1] + (vertex_input.vertex_position[1] * vertex_input.instance_area[1]),
        vertex_input.instance_depth,
        1.0);
    return VertexOutput(viewport.view_matrix * coordinates, vertex_input.instance_rasterization_descriptor,
    vertex_input.instance_color, vertex_input.instance_position);
}
@group(1)
@binding(1)
var<storage, read> glyph_rasterization: array<u32>;
@fragment
fn fragment_entry(
    fragment_input: FragmentInput,
) -> @location(0) vec4<f32> {
    let rasterization_px = fragment_input.position - vec4<f32>(fragment_input.instance_position.xy, 0.0, 0.0);
    let row_index = rasterization_px.y * f32(fragment_input.instance_rasterization_descriptor.y);
    let rasterization_offset = row_index + rasterization_px.x;
    let coverage = glyph_rasterization[u32(rasterization_offset)];
    let normalized_coverage = coverage / 255u;
    let coverage_applied_color = vec4<f32>(fragment_input.color.rgb, fragment_input.color.a * f32(normalized_coverage));
    return coverage_applied_color;
}