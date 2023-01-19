struct Viewport {
    view_matrix: mat4x4<f32>,
};
@group(0)
@binding(0)
var<uniform> viewport: Viewport;
struct Instance {

};
struct VertexInput {
    @builtin(vertex_index) vertex_index: u32,
    @location(0) vertex_position: vec2<f32>,
    @location(1) position: vec2<f32>,
    @location(2) area: vec2<f32>,
    @location(3) depth: f32,
    @location(4) color: vec4<f32>,
    @location(5) tex_coords: vec4<f32>,
};
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) @interpolate(flat) color: vec4<f32>,
    @location(1) tex_coords: vec4<f32>,
};
@vertex
fn vertex_entry(
    vertex_input: VertexInput,
) -> VertexOutput {
    let coordinates = vec4<f32>(
        vertex_input.instance.position.x + (vertex_input.vertex_position.x * vertex_input.instance.area.x),
        vertex_input.instance.position.y + (vertex_input.vertex_position.y * vertex_input.instance.area.y),
        vertex_input.instance.depth,
        1.0);
    return VertexOutput(viewport.view_matrix * coordinates, vertex_input.instance.color, vertex_input.instance.tex_coords);
}
@group(1)
@binding(0)
var rasterization_sampler: sampler;
@group(2)
@binding(0)
var rasterization_tex: texture_2d<u32>;
@fragment
fn fragment_entry(
    fragment_input: VertexOutput,
) -> @location(0) vec4<f32> {
    let coverage = textureSample(rasterization_tex, rasterization_sampler, fragment_input.tex_coords).r;
    if (coverage == 0) {
        discard;
    }
    let alpha = f32(coverage) / 255.0;
    return vec4<f32>(fragment_input.color.rgb, fragment_input.color.a * alpha);
}