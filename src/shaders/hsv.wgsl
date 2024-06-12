struct HSV {
    hue: f32,
    saturation: f32,
    value: f32,
}

@group(0) @binding(0) var<uniform> hsv: HSV;

@fragment
fn fs_main(
    @builtin(position) _clip_pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
) -> @location(0) vec4<f32> {
    // uv.x = saturation
    // uv.y = value

    // HSV to RGB
    let c = uv.y * uv.x;
    let h_ = hsv.hue / 60.0;
    let x = c * (1.0 - abs(h_ % 2.0 - 1.0));

    var r1 = 0.0;
    var g1 = 0.0;
    var b1 = 0.0;
    if 0.0 <= h_ && h_ < 1.0 {
        r1 = c;
        g1 = x;
    } else if 1.0 <= h_ && h_ < 2.0 {
        r1 = x;
        g1 = c;
    } else if 2.0 <= h_ && h_ < 3.0 {
        g1 = c;
        b1 = x;
    } else if 3.0 <= h_ && h_ < 4.0 {
        g1 = x;
        b1 = c;
    } else if 4.0 <= h_ && h_ < 5.0 {
        r1 = x;
        b1 = c;
    } else {
        r1 = c;
        b1 = x;
    }

    let m = uv.y - c;
    let color = vec4<f32>(r1 + m, g1 + m, b1 + m, 1.0);
    return color;
}
