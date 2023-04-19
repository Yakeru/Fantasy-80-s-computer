// Vertex shader bindings

struct VertexOutput {
    @location(0) tex_coord: vec2<f32>,
    @builtin(position) position: vec4<f32>,
}

@vertex
fn vs_main(
    @location(0) position: vec2<f32>,
) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coord = fma(position, vec2<f32>(0.5, -0.5), vec2<f32>(0.5, 0.5));
    out.position = vec4<f32>(position, 0.0, 1.0);
    return out;
}

// Fragment shader bindings

@group(0) @binding(0) var r_tex_color: texture_2d<f32>;
@group(0) @binding(1) var r_tex_sampler: sampler;
// struct Locals {
//     screen_width: f32,
//     screen_height: f32
// }
@group(0) @binding(2) var<uniform> screen_width: f32;
@group(0) @binding(3) var<uniform> screen_height: f32;

// Amount of shadow mask.
const maskDark = 0.2;
const maskLight = 1.0;
const maskSize = 3.0;

const blurLevel = 4.0;
const scanlineWidth = 6.0;
const scanlineStrength = 0.4;

fn mask(pos: vec2<f32>) -> vec3<f32> {
    var pos_x = pos.x + pos.y * 3.0;
    var mask: vec3<f32> = vec3<f32>(maskDark, maskDark, maskDark);
    pos_x = fract(pos_x / maskSize);
    if (pos_x < 0.333) {mask.r = maskLight;}
    else if (pos_x < 0.666) {mask.g = maskLight;}
    else {mask.b = maskLight;}
    return mask;
}

fn get_screen_coord(tex_coord: vec2<f32>) -> vec2<f32> {
    return vec2<f32>(floor(tex_coord[0] * screen_width), floor(tex_coord[1] * screen_height));
}

@fragment
fn fs_main(@location(0) tex_coord: vec2<f32>) -> @location(0) vec4<f32> {
    
    var sampled_color = textureSample(r_tex_color, r_tex_sampler, tex_coord);

    var left_coord = tex_coord - vec2<f32>((1.0/680.0), 0.0);
    var sampled_color_left = textureSample(r_tex_color, r_tex_sampler, left_coord);

    var right_coord = tex_coord + vec2<f32>((1.0/680.0), 0.0);
    var sampled_color_right = textureSample(r_tex_color, r_tex_sampler, right_coord);

    var screen_coord = get_screen_coord(tex_coord);

    sampled_color = (sampled_color * (blurLevel - 2.0 ) + sampled_color_left + sampled_color_right) / blurLevel;

    if (screen_coord.y % scanlineWidth == 0.0) {
        sampled_color *= scanlineStrength * 0.1;
    } else if ((screen_coord.y + 1.0) % scanlineWidth == 0.0 || (screen_coord.y - 1.0) % scanlineWidth == 0.0) {
        sampled_color *= scanlineStrength * 0.5;
    } else if ((screen_coord.y + 2.0) % scanlineWidth == 0.0 || (screen_coord.y - 2.0) % scanlineWidth == 0.0) {
        sampled_color *= scanlineStrength;
    }

    return vec4<f32>(sampled_color.rgb * mask(screen_coord), sampled_color.a);
}