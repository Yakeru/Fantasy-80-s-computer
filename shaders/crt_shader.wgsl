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
struct Locals {
    screen_width: f32,
    screen_height: f32,
    mode: f32,
    scanline_height: f32,
    mask_size: f32,
    mask_type: f32,
    horiz_distortion: f32,
    vert_distortion: f32
}
@group(0) @binding(2) var<uniform> r_locals: Locals;

// Amount of shadow mask.
const maskDark = 0.3;
const maskLight = 1.0;
const blurLevel = 3.0;

fn mask(pos: vec2<f32>) -> vec3<f32> {
    var pos_x = pos.x + pos.y * r_locals.mask_type;
    var mask: vec3<f32> = vec3<f32>(maskDark, maskDark, maskDark);
    pos_x = fract(pos_x / r_locals.mask_size);
    if (pos_x < 0.333) {mask.r = maskLight;}
    else if (pos_x < 0.666) {mask.g = maskLight;}
    else {mask.b = maskLight;}
    return mask;
}

fn get_screen_coord(tex_coord: vec2<f32>) -> vec2<f32> {
    return vec2<f32>(floor(tex_coord[0] * r_locals.screen_width), floor(tex_coord[1] * r_locals.screen_height));
}

// Distortion of scanlines, and end of screen alpha.
fn warp(pos: vec2<f32>) -> vec2<f32>{
    if (r_locals.horiz_distortion == 0.0 || r_locals.vert_distortion == 0.0) {
        return pos;
    }

    let warp: vec2<f32> = vec2<f32>(1.0/r_locals.horiz_distortion, 1.0/r_locals.vert_distortion);
    var new_pos: vec2<f32> = pos*2.0 - 1.0;    
    new_pos *= vec2<f32>(1.0+(new_pos.y*new_pos.y)*warp.x, 1.0+(new_pos.x*new_pos.x)*warp.y);
    return new_pos*0.5 + 0.5;
}

fn warped_coord_outsie_screen(warped_coord: vec2<f32>) -> bool {
    if(max(abs(warped_coord.x - 0.5), abs(warped_coord.y - 0.5))>0.5) {
        return true;
    }
    return false;
}

@fragment
fn fs_main(@location(0) tex_coord: vec2<f32>) -> @location(0) vec4<f32> {
    
    var warped_coord = warp(tex_coord);
    var warped_left_coord = warp(tex_coord - vec2<f32>((1.0/680.0), 0.0));
    var warped_right_coord = warp(tex_coord + vec2<f32>((1.0/680.0), 0.0));

    var sampled_color = textureSample(r_tex_color, r_tex_sampler, warped_coord);
    var sampled_color_left = textureSample(r_tex_color, r_tex_sampler, warped_left_coord);
    var sampled_color_right = textureSample(r_tex_color, r_tex_sampler, warped_right_coord);

    if(warped_coord_outsie_screen(warped_coord)) {sampled_color *= 0.0;}
    if(warped_coord_outsie_screen(warped_left_coord)) {sampled_color_left *= 0.0;}
    if(warped_coord_outsie_screen(warped_right_coord)) {sampled_color_right *= 0.0;}

    var screen_coord = get_screen_coord(tex_coord);
    var screen_coord_warped = get_screen_coord(warped_coord);

    sampled_color = (sampled_color * blurLevel + sampled_color_left + sampled_color_right) / (blurLevel + 2.0);

    if (screen_coord_warped.y % r_locals.scanline_height == 0.0) {
        sampled_color *= 0.0;
    } else if ((screen_coord_warped.y + 1.0) % r_locals.scanline_height == 0.0 || (screen_coord_warped.y - 1.0) % r_locals.scanline_height == 0.0) {
        sampled_color *= 0.0;
    } else if ((screen_coord_warped.y + 2.0) % r_locals.scanline_height == 0.0 || (screen_coord_warped.y - 2.0) % r_locals.scanline_height == 0.0) {
        sampled_color *= 0.4;
    }

    var pixel: vec3<f32> = sampled_color.rgb * mask(screen_coord_warped);
    
    if (r_locals.mode == 1.0) {
        pixel = vec3<f32>(pixel.r + pixel.g + pixel.b) / 3.0 * vec3<f32>(1.0, 0.4, 0.0);
    }

    if (r_locals.mode == 2.0) {
        pixel = vec3<f32>(pixel.r + pixel.g + pixel.b) / 3.0 * vec3<f32>(0.1, 1.0, 0.1);
    }
    
    return vec4<f32>(pixel, sampled_color.a);
}