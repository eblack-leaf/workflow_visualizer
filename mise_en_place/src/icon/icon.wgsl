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
    @location(2) area: vec2<f32>,
    @location(3) depth: f32,
    @location(4) color: vec4<f32>,
    @location(5) null_bit: u32,
    @location(6) color_invert: u32,
};
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
};
fn hookable(vertex_data: vec4<f32>) -> bool {
    return vertex_data.a == 1.0;
}
fn negative_space(vertex_data: vec4<f32>) -> bool {
    return vertex_data.b == 1.0;
}
@vertex
fn vertex_entry(vertex_input: VertexInput) -> VertexOutput {
    let nulled = bool(vertex_input.null_bit) == true;
    let null_mult = f32(!nulled);
    var coordinates = vec4<f32>(
        vertex_input.position.x + vertex_input.vertex_data.x * vertex_input.area.x,
        vertex_input.position.y + vertex_input.vertex_data.y * vertex_input.area.y,
        vertex_input.depth,
        1.0);
    var offset_coordinates = vec4<f32>(coordinates.rg - viewport_offset.rg, coordinates.ba);
    var resolved_color = vertex_input.color;
    if (hookable(vertex_input.vertex_data)) {
        if (vertex_input.color_invert == u32(1)) {
            if (negative_space(vertex_input.vertex_data)) {
                resolved_color = vertex_input.color;
            } else {
                resolved_color = vec4<f32>(0.0, 0.0, 0.0, 0.0);
            }
        } else {
            if (negative_space(vertex_input.vertex_data)) {
                resolved_color = vec4<f32>(0.0, 0.0, 0.0, 0.0);
            } else {
                resolved_color = vertex_input.color;
            }
        }
    } else {
        if (negative_space(vertex_input.vertex_data)) {
            resolved_color = vec4<f32>(0.0, 0.0, 0.0, 0.0);
        } else {
            resolved_color = vertex_input.color;
        }
    }
    return VertexOutput(viewport.view_matrix * offset_coordinates * null_mult, resolved_color);
}
@fragment
fn fragment_entry(vertex_output: VertexOutput) -> @location(0) vec4<f32> {
    return vertex_output.color;
}