struct Viewport {
    view_matrix: mat4x4<f32>,
};
@group(0)
@binding(0)
var<uniform> viewport: Viewport;
@group(0)
@binding(1)
var<uniform> viewport_offset: vec4<f32>;
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
struct VertexInput {
    @builtin(vertex_index) vertex_index: u32,
    @location(0) vertex_position: vec2<f32>,
};
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) sample_coords: vec2<f32>,
    @location(1) fade: f32,
};
@group(3)
@binding(0)
var<uniform> fade_and_layer: vec4<f32>;
@group(3)
@binding(1)
var<uniform> texture_coordinates: vec4<f32>;
@group(3)
@binding(2)
var<uniform> placement: vec4<f32>;
@vertex
fn vertex_entry(vertex_input: VertexInput) -> VertexOutput {
    let coordinates = vec4<f32>(placement.xy - viewport_offset.xy + vertex_input.vertex_position.xy * placement.zw, fade_and_layer.g, 1.0);
    let sample_coordinates = adjust_coords_of(vertex_input.vertex_index, texture_coordinates);
    let output = VertexOutput(
        viewport.view_matrix * coordinates,
        sample_coordinates,
        fade_and_layer.r,
    );
    return output;
}
@group(1)
@binding(0)
var image_sampler: sampler;
@group(2)
@binding(0)
var image_texture: texture_2d<f32>;
@group(3)
@binding(3)
var<uniform> icon_color: vec4<f32>;
@fragment
fn fragment_entry(vertex_output: VertexOutput) -> @location(0) vec4<f32> {
    let image_data = textureSample(image_texture, image_sampler, vertex_output.sample_coords);
    var color = vec4<f32>(image_data.rgba);
    if (icon_color.a <= 0.0) {
        let alpha = image_data.a * vertex_output.fade;
        if (alpha <= 0.0) {
            discard;
        }
        color = vec4<f32>(image_data.rgb, alpha);
    } else {
        let alpha = image_data.a * icon_color.a;
        if (alpha <= 0.0) { discard; }
        color = vec4<f32>(icon_color.rgb, alpha);
    }
    return color;
}