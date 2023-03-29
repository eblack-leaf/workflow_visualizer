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
    @location(2) content_area: vec2<f32>,
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
    let offset_pos = vertex_input.position - viewport_offset.xy;
    let content_scaled = offset_pos + vertex_input.vertex_data.xy + vertex_input.content_area * vertex_input.vertex_data.zw;
    let vertex_out_pos = vec4<f32>(content_scaled, vertex_input.layer, 1.0);
    return VertexOutput(viewport.view_matrix * vertex_out_pos * null_mult, vertex_input.color);
}
fn noise_rand(x: f32) -> f32 {
    return fract(sin(x) * 43759.473);
}

fn noise_permute_f32(x: f32) -> f32 {
    return (((x * 34.0) + 10.0) * x) % 289.0;
}

fn noise_fade_f32(x: f32) -> f32 {
    return x * x * x * (x * (x * 6.0 - 15.0) + 10.0);
}

fn noise_permute_vec2f(x: vec2<f32>) -> vec2<f32> {
    return (((x * 34.0) + 10.0) * x) % 289.0;
}

fn noise_fade_vec2f(x: vec2<f32>) -> vec2<f32> {
    return x * x * x * (x * (x * 6.0 - 15.0) + 10.0);
}

fn noise_permute_vec3f(x: vec3<f32>) -> vec3<f32> {
    return (((x * 34.0) + 10.0) * x) % 289.0;
}

fn noise_fade_vec3f(x: vec3<f32>) -> vec3<f32> {
    return x * x * x * (x * (x * 6.0 - 15.0) + 10.0);
}

fn noise_permute_vec4f(x: vec4<f32>) -> vec4<f32> {
    return (((x * 34.0) + 10.0) * x) % 289.0;
}

fn noise_fade_vec4f(x: vec4<f32>) -> vec4<f32> {
    return x * x * x * (x * (x * 6.0 - 15.0) + 10.0);
}
fn noise_perlin_vec2f(p: vec2<f32>) -> f32 {
    var pi = floor(p.xyxy) + vec4(0.0, 0.0, 1.0, 1.0);
    pi = pi % 289.0;    // to avoid trauncation effects in permutation

    let pf = fract(p.xyxy) - vec4(0.0, 0.0, 1.0, 1.0);

    let ix = pi.xzxz;
    let iy = pi.yyww;
    let fx = pf.xzxz;
    let fy = pf.yyww;

    let i = noise_permute_vec4f(noise_permute_vec4f(ix) + iy);

    var gx = fract(i * (1.0 / 41.0)) * 2.0 - 1.0;
    let gy = abs(gx) - 0.5 ;
    let tx = floor(gx + 0.5);
    gx = gx - tx;

    var g00 = vec2(gx.x, gy.x);
    var g10 = vec2(gx.y, gy.y);
    var g01 = vec2(gx.z, gy.z);
    var g11 = vec2(gx.w, gy.w);

    let norm = inverseSqrt(vec4(
        dot(g00, g00),
        dot(g01, g01),
        dot(g10, g10),
        dot(g11, g11)
    ));
    g00 *= norm.x;
    g01 *= norm.y;
    g10 *= norm.z;
    g11 *= norm.w;

    let n00 = dot(g00, vec2(fx.x, fy.x));
    let n10 = dot(g10, vec2(fx.y, fy.y));
    let n01 = dot(g01, vec2(fx.z, fy.z));
    let n11 = dot(g11, vec2(fx.w, fy.w));

    let fade_xy = noise_fade_vec2f(pf.xy);
    let n_x = mix(vec2(n00, n01), vec2(n10, n11), fade_xy.x);
    let n_xy = mix(n_x.x, n_x.y, fade_xy.y);
    return 2.3 * n_xy;
}
fn noise_simplex_vec2f(v: vec2<f32>) -> f32 {
    let c = vec4(
        0.211324865405187,      // (3.0-sqrt(3.0))/6.0
        0.366025403784439,      // 0.5*(sqrt(3.0)-1.0)
        -0.577350269189626,     // -1.0 + 2.0 * C.x
        0.024390243902439       // 1.0 / 41.0
    );

    // First corner
    var i = floor(v + dot(v, c.yy));
    let x0 = v - i + dot(i, c.xx);

    // Other corners
    var i1: vec2<f32>;
    // i1.x = step( x0.y, x0.x ); = x0.x > x0.y ? 1.0 : 0.0
    // i1.y = 1.0 - i1.x;
    if x0.x > x0.y {
        i1 = vec2(1.0, 0.0);
    } else {
        i1 = vec2(0.0, 1.0);
    }

    // x0 = x0 - 0.0 + 0.0 * C.xx ;
    // x1 = x0 - i1 + 1.0 * C.xx ;
    // x2 = x0 - 1.0 + 2.0 * C.xx ;
    var x12 = x0.xyxy + c.xxzz;
    let sw0 = x12.xy - i1;
    x12.x = sw0.x;
    x12.y = sw0.y;

    // Permutations
    i = i % 289.0;          // Avoid truncation effects in permutation
    let p = noise_permute_vec3f(
        noise_permute_vec3f(i.y + vec3(0.0, i1.y, 1.0)) + i.x + vec3(0.0, i1.x, 1.0)
    );

    var m = max(
        0.5 - vec3(dot(x0, x0), dot(x12.xy, x12.xy), dot(x12.zw, x12.zw)),
        vec3(0.0)
    );
    m = m * m ;
    m = m * m ;

    // Gradients: 41 points uniformly over a line, mapped onto a diamond.
    // The ring size 17*17 = 289 is close to a multiple of 41 (41*7 = 287)
    let x = 2.0 * fract(p * c.www) - 1.0;
    let h = abs(x) - 0.5;
    let ox = floor(x + 0.5);
    let a0 = x - ox;

    // Normalise gradients implicitly by scaling m
    // Approximation of: m *= inversesqrt( a0*a0 + h*h );
    m *= 1.79284291400159 - 0.85373472095314 * (a0 * a0 + h * h);

    // Compute final noise value at P
    var g: vec3<f32>;
    g.x = a0.x * x0.x + h.x * x0.y;
    g.y = a0.y * x12.x + h.y * x12.y;
    g.z = a0.z * x12.z + h.z * x12.w;
    return 130.0 * dot(m, g);
}
@fragment
fn fragment_entry(vertex_output: VertexOutput) -> @location(0) vec4<f32> {
    let noise = noise_simplex_vec2f(-vertex_output.position.xy / 1080.0);
    let adjusted = vec4<f32>(vertex_output.color.rgb - vertex_output.color.rgb * noise*.25, vertex_output.color.a);
    let normal = vertex_output.color;
    return normal;
}
