// this is the shader
// all html components will use the same shader

// sort of like a camera
struct ScreenInfo{
    width: u32,
    height: u32,
    scroll_offsets: vec2<f32>
};
 
struct ComponentInput{
    @location(0) position: vec2<f32>,
    //@location(1) color_or_texture: f32, // 0 false, 1 true
    //@location(2) color_or_texture_position: vec3<f32> // should be [x, y, 0.0] if texture pos
}

struct FragmentInput{
    @builtin(position) clip_position: vec4<f32>,
    @location(1) color: vec4<f32>
}

// Vertex Shader

@vertex
fn vs_main(input: ComponentInput) -> FragmentInput {
    // adjust position based on Scroll
    var out: FragmentInput;
    out.clip_position = vec4(input.position, 0.0, 1.0);
    // doing colors
    out.color = vec4(1.0, 1.0, 1.0, 1.0);
    return out;
}

// Fragment Shader

@fragment
fn fs_main(frag: FragmentInput) -> @location(0) vec4<f32>{
    return vec4(1.0, 1.0, 1.0, 1.0);
} 