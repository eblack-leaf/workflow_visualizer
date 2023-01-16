struct Viewport {
    view_matrix: mat4x4<f32>,
};
@group(0)
@binding(0)
var<uniform> viewport: Viewport;
struct VertexInput {
    @builtin(vertex_index) vertex_index: u32,
    @location(0) vertex_position: vec2<f32>,
    @location(1) instance_position: vec2<f32>,
    @location(2) instance_area: vec2<f32>,
    @location(3) instance_depth: f32,
    @location(4) instance_color: vec4<f32>,
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
        vertex_input.instance_position.x + (vertex_input.vertex_position.x * vertex_input.instance_area.x),
        vertex_input.instance_position.y + (vertex_input.vertex_position.y * vertex_input.instance_area.y),
        vertex_input.instance_depth,
        1.0);
    return VertexOutput(viewport.view_matrix * coordinates, vertex_input.instance_color, vertex_input.tex_coords,
    );
}
@group(1)
@binding(0)
var rasterization_tex: texture_2d<u32>;
@group(1)
@binding(1)
var rasterization_sampler: sampler;
@fragment
fn fragment_entry(
    fragment_input: VertexOutput,
) -> @location(0) vec4<f32> {
    let coverage = textureSample(rasterization_tex, rasterization_sampler, fragment_input.tex_coords).r;
    if (coverage == 0) {
        discard;
    }

}