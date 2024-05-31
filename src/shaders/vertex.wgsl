struct Output {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> Output {
    let uv = vec2<f32>(vec2<u32>((vertex_index << 1) & 2, vertex_index & 2));
    let position = vec4<f32>(uv * 2 - 1, 0, 1);
    return Output(position, uv);
}
