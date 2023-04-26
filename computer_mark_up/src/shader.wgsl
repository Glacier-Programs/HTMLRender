// this is the shader
// all html components will use the same shader

// sort of like a camera
struct Scroll(vec2<f32>);
 
struct ComponentInput{
    @location(0) position: vec2<f32>,
    @location(1) color_or_texture: f32, // 0 false, 1 true
    @location(2) color_or_texture_position: vec3<f32> // should be [x, y, 0.0] if texture pos
}

struct FragmentInput{
    @builtin(position) clip_position: vec3<f32>,
    @location(1) color: vec4<f32>
}

// Vertex Shader

@vertex
fn vs_main(input: ComponentInput) -> FragmentInput {
    // adjust position based on Scroll
    var out: FragmentInput;
    out.clip_position = vec3(input.position, 0.0);
    // doing colors
}

// Fragment Shader

@fragment
fn fs_main(frag: FragmentInput) -> vec4<f32>{

} 