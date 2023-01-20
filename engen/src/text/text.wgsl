struct Viewport {
    view_matrix: mat4x4<f32>,
};
@group(0)
@binding(0)
var<uniform> viewport: Viewport;
struct VertexInput {
    @builtin(vertex_index) vertex_index: u32,
    @location(0) vertex_position: vec2<f32>,
    @location(1) position: vec2<f32>,
    @location(2) depth: f32,
    @location(3) color: vec4<f32>,
    @location(4) area: vec2<f32>,
    @location(5) tex_coords: vec4<f32>,
};
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) @interpolate(flat) color: vec4<f32>,
    @location(1) tex_coords: vec2<f32>,
};
@vertex
fn vertex_entry(
    vertex_input: VertexInput,
) -> VertexOutput {
    let coordinates = vec4<f32>(
        vertex_input.position.x + (vertex_input.vertex_position.x * vertex_input.area.x),
        vertex_input.position.y + (vertex_input.vertex_position.y * vertex_input.area.y),
        vertex_input.depth,
        1.0);
        var adjusted_coords = vec2<f32>(0.0, 0.0);
        var top = vertex_input.tex_coords.g;
        var left = vertex_input.tex_coords.r;
        var bottom = vertex_input.tex_coords.a;
        var right = vertex_input.tex_coords.b;
        switch (i32(vertex_input.vertex_index)) {
                case 0: {
                    // top left
                    adjusted_coords = vec2<f32>(left, top);
                }
                case 1: {
                    // bottom left
                    adjusted_coords = vec2<f32>(left, bottom);
                }
                case 2: {
                    // top right
                    adjusted_coords = vec2<f32>(right, top);
                }
                case 3: {
                    // top right
                    adjusted_coords = vec2<f32>(right, top);
                }
                case 4: {
                    // bottom left
                    adjusted_coords = vec2<f32>(left, bottom);
                }
                case 5: {
                    // bottom right
                    adjusted_coords = vec2<f32>(right, bottom);
                }
                default: {}
            }
    return VertexOutput(viewport.view_matrix * coordinates, vertex_input.color, adjusted_coords);
}
@group(1)
@binding(0)
var rasterization_sampler: sampler;
@group(2)
@binding(0)
var rasterization_tex: texture_2d<f32>;
@fragment
fn fragment_entry(
    fragment_input: VertexOutput,
) -> @location(0) vec4<f32> {
    let coverage = textureSample(rasterization_tex, rasterization_sampler, fragment_input.tex_coords).r;
    if (coverage == 0.0) {
        discard;
    }
    return vec4<f32>(fragment_input.color.rgb, fragment_input.color.a * coverage);
}