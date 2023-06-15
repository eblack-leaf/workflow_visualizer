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
    @builtin(vertex_index) vertex_index: u32,
    @location(0) vertex_position: vec2<f32>,
    @location(1) position: vec2<f32>,
    @location(2) area: vec2<f32>,
    @location(3) layer: f32,
    @location(4) color: vec4<f32>,
    @location(5) tex_coords: vec4<f32>,
    @location(6) null_bit: u32,
};
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) sample_coords: vec2<f32>,
    @location(1) color: vec4<f32>,
};
fn adjust_coords_of(vertex_index: u32, tex_coords: vec4<f32>) -> vec2<f32> {
    var adjusted_coords = vec2<f32>(0.0, 0.0);
    var top = tex_coords.g;
    var left = tex_coords.r;
    var bottom = tex_coords.a;
    var right = tex_coords.b;
    switch (i32(vertex_index)) {
        case 0: {
            adjusted_coords = vec2<f32>(left, top);
        }
        case 1: {
            adjusted_coords = vec2<f32>(left, bottom);
        }
        case 2: {
            adjusted_coords = vec2<f32>(right, top);
        }
        case 3: {
            adjusted_coords = vec2<f32>(right, top);
        }
        case 4: {
            adjusted_coords = vec2<f32>(left, bottom);
        }
        case 5: {
            adjusted_coords = vec2<f32>(right, bottom);
        }
        default: {}
    }
    return adjusted_coords;
}
@vertex
fn vertex_entry(vertex_input: VertexInput) -> VertexOutput {
    let nulled = bool(vertex_input.null_bit) == true;
    let null_mult = f32(!nulled);
    let coordinates = vec4<f32>(vertex_input.position - viewport_offset.xy + vertex_input.vertex_position * vertex_input.area, vertex_input.layer, 1.0);
    let sample_coordinates = adjust_coords_of(vertex_input.vertex_index, vertex_input.tex_coords);
    let output = VertexOutput(
        viewport.view_matrix * coordinates * null_mult,
        sample_coordinates,
        vertex_input.color
    );
    return output;
}
@group(1)
@binding(0)
var icon_texture: texture_2d<f32>;
@group(1)
@binding(1)
var icon_sampler: sampler;
@fragment
fn fragment_entry(fragment_input: VertexOutput) -> @location(0) vec4<f32> {
    let coverage = textureSample(icon_texture, icon_sampler, fragment_input.sample_coords).r;
    if (coverage == 0.0) {
        discard;
    }
    return vec4<f32>(fragment_input.color.rgb, fragment_input.color.a * coverage);
}