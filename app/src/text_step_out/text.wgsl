@group(0)
@binding(0)
var<uniform> viewport: Viewport;
struct Viewport {
    view_matrix: mat4x4<f32>,
};
struct VertexInput {
    @builtin(position) vertex_position: vec2<f32>,
    @location(0) instance_color: vec4<f32>,
    @location(1) instance_position: vec2<f32>,
    @location(2) instance_area: vec2<f32>,
    @location(3) instance_depth: f32,
    @location(4) instance_rasterization_descriptor: vec3<u32>,

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
        instance_position[0] + (vertex_position[0] * instance_area[0]),
        instance_position[1] + (vertex_position[1] * instance_area[1]),
        instance_depth,
        1.0);
    return VertexOutput(viewport * coordinates, instance_rasterization_descriptor, instance_color, instance_position);
}
@group(1)
@binding(0)
var<storage, read> glyph_rasterization: array<u8>;
@fragment
fn fragment_entry(
    fragment_input: FragmentInput,
) -> @location vec4<f32> {
    let rasterization_px = fragment_input.position - fragment_input.instance_position;
    let row_index = rasterization_px.y * fragment_input.instance_rasterization_descriptor.y;
    let rasterization_offset = row_index + rasterization_px.x;
    let coverage = glyph_rasterization[rasterization_offset];
    let normalized_coverage = coverage / 255;
    let coverage_applied_color = vec4<f32>(fragment_input.color.rgb, fragment_input.color.a * normalized_coverage);
    return coverage_applied_color;
}