struct OKLCH {
    lightness: f32,
    chroma: f32,
    hue: f32,
    mode: u32,
}

const MODE_LIGHTNESS = 0u;
const MODE_CHROMA = 1u;
const MODE_HUE = 2u;

@group(0) @binding(0) var<uniform> oklch: OKLCH;

@fragment
fn fs_main(
    @builtin(position) _clip_pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
) -> @location(0) vec4<f32> {
    var rgb = vec3<f32>(0.0);
    switch oklch.mode {
        case MODE_LIGHTNESS: {
            let chroma = uv.y * 0.37;
            let lightness = uv.x;
            
            rgb = oklch_to_rgb(lightness, chroma, oklch.hue);
        }
        case MODE_CHROMA: {
            let chroma = uv.y * 0.37;
            let hue = uv.x * 360.0;

            rgb = oklch_to_rgb(oklch.lightness, chroma, hue);
        }
        case MODE_HUE: {
            let lightness = uv.y;
            let hue = uv.x * 360.0;

            rgb = oklch_to_rgb(lightness, oklch.chroma, hue);
        }
        default: {}
    }

    var color = vec4<f32>(rgb, 1.0);
    if max(color.x, max(color.y, color.z)) > 1.0 || min(color.x, min(color.y, color.z)) < 0.0 {
        color.w = 0.0;
    }
    return color;
}

fn oklch_to_rgb(okl: f32, okc: f32, okh: f32) -> vec3<f32> {
    let h = radians(okh);
    let a = okc * cos(h);
    let b = okc * sin(h);

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
