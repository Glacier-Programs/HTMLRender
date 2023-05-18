// this is the shader
// all html components will use the same shader

// The top left corner will be treated as (0,0)
// since this makes it easier to visualize scrolling down

// sort of like a camera
struct ScreenInfo{
    width: u32,
    height: u32,
    scroll_offsets: vec2<f32>
};

struct ComponentInput{
    @location(0) position: vec2<f32>,
    @location(1) texture_index: u32,
    @location(2) texture_position: vec2<f32>
    //@location(1) color_or_texture: f32, // 0 false, 1 true
    //@location(2) color_or_texture_position: vec3<f32> // should be [x, y, 0.0] if texture pos
}

struct FragmentInput{
    @builtin(position) clip_position: vec4<f32>,
    @location(1) texture_position: vec2<f32>,
    @location(2) texture: u32
}

// Vertex Shader

@group(0) @binding(0)
var<uniform> SCREENDETAILS: ScreenInfo;

// Texture data
@group(1) @binding(0)
var texture_data: texture_2d<f32>;
@group(1)@binding(1)
var texture_sampler: sampler;

@vertex
fn vs_main(input: ComponentInput) -> FragmentInput {
    // adjust position based on Scroll and format to screen
    var out: FragmentInput;
    let adjusted_x = input.position[0] / f32(SCREENDETAILS.width) * 2.0 - 1.0;
    let adjusted_y = input.position[1] / f32(SCREENDETAILS.height) * 2.0;
    out.clip_position = vec4(adjusted_x, adjusted_y, 0.0, 1.0);
    // doing colors
    out.texture_position = input.texture_position;
    out.texture = input.texture_index;
    return out;
}

// Fragment Shader

@fragment
fn fs_main(frag: FragmentInput) -> @location(0) vec4<f32>{
    return textureSample(texture_data, texture_sampler, frag.texture_position);
}