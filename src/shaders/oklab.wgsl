struct OKLAB {
    lightness: f32,
    green_red: f32,
    blue_yellow: f32,
    mode: u32,
}

const MODE_LIGHTNESS = 0u;
const MODE_GREEN_RED = 1u;
const MODE_BLUE_YELLOW = 2u;

@group(0) @binding(0) var<uniform> oklab: OKLAB;

@fragment
fn fs_main(
    @builtin(position) _clip_pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
) -> @location(0) vec4<f32> {
    var rgb = vec3<f32>(0.0);
    switch oklab.mode {
        case MODE_LIGHTNESS: {
            let green_red = uv.y - 0.5;
            let lightness = uv.x;

            rgb = oklab_to_rgb(lightness, green_red, oklab.blue_yellow);
            
        }
        case MODE_GREEN_RED: {
            let green_red = uv.y - 0.5;
            let blue_yellow = uv.x - 0.5;

            rgb = oklab_to_rgb(oklab.lightness, green_red, blue_yellow);
        }
        case MODE_BLUE_YELLOW: {
            let lightness = uv.y;
            let blue_yellow = uv.x - 0.5;

            rgb = oklab_to_rgb(lightness, oklab.green_red, blue_yellow);
        }
        default: {}
    }

    var color = vec4<f32>(rgb, 1.0);
    if max(color.x, max(color.y, color.z)) > 1.0 || min(color.x, min(color.y, color.z)) < 0.0 {
        color.w = 0.1;
    }
    return color;
}

fn oklab_to_rgb(okl: f32, a: f32, b: f32) -> vec3<f32> {
    let l_ = okl + 0.3963377774 * a + 0.2158037573 * b;
    let m_ = okl - 0.1055613458 * a - 0.0638541728 * b;
    let s_ = okl - 0.0894841775 * a - 1.2914855480 * b;

    let l = l_ * l_ * l_;
    let m = m_ * m_ * m_;
    let s = s_ * s_ * s_;

    return vec3<f32>(
        4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s,
        -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s,
        -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s,
    );
}
