// This shader computes the chromatic aberration effect

// Since post processing is a fullscreen effect, we use the fullscreen vertex shader provided by bevy.
// This will import a vertex shader that renders a single fullscreen triangle.
//
// A fullscreen triangle is a single triangle that covers the entire screen.
// The box in the top left in that diagram is the screen. The 4 x are the corner of the screen
//
// Y axis
//  1 |  x-----x......
//  0 |  |  s  |  . ´
// -1 |  x_____x´
// -2 |  :  .´
// -3 |  :´
//    +---------------  X axis
//      -1  0  1  2  3
//
// As you can see, the triangle ends up bigger than the screen.
//
// You don't need to worry about this too much since bevy will compute the correct UVs for you.
#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

// ? struct to remember types
// struct FullscreenVertexOutput {
//     @builtin(position)
//     position: vec4<f32>,
//     @location(0)
//     uv: vec2<f32>,
// };

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;

struct PostProcessSettings {
    intensity: f32,
// #ifdef SIXTEEN_BYTE_ALIGNMENT
//     // WebGL2 structs must be 16 byte aligned.
//     _webgl2_padding: vec3<f32>
// #endif
}
@group(0) @binding(2) var<uniform> settings: PostProcessSettings;

const PI2 = 6.28318530718;

// GAUSSIAN BLUR SETTINGS
    const DIRECTIONS: f32 = 10.; // BLUR DIRECTIONS (Default 16.0 - More is better but slower)
    const QUALITY: f32 = 3.; // BLUR QUALITY (Default 4.0 - More is better but slower)
    const SIZE: f32 = 2.; // BLUR SIZE (Radius)
    //

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    // Chromatic aberration strength
    let offset_strength = 0.0025; //settings.intensity;

    // Sample each color channel with an arbitrary shift
    let color_chab = vec4<f32>(
        textureSample(screen_texture, texture_sampler, in.uv + vec2<f32>(offset_strength, -offset_strength)).r,
        textureSample(screen_texture, texture_sampler, in.uv + vec2<f32>(-offset_strength, 0.0)).g,
        textureSample(screen_texture, texture_sampler, in.uv + vec2<f32>(0.0, offset_strength)).b,
        1.0
    );

    var color_blur = textureSample(screen_texture, texture_sampler, in.uv);

    let dimensions = textureDimensions(screen_texture);

    let radius: vec2<f32> = SIZE / vec2f(dimensions.xy);

    for (var d = 0.0; d < PI2; d += PI2 / DIRECTIONS) {
        for (var i = 1.0 / QUALITY; i <= 1.0; i += 1.0 / QUALITY) {
            color_blur += textureSample(screen_texture, texture_sampler, in.uv + vec2f(cos(d), sin(d)) * radius * i);
        }
    }

    color_blur /= QUALITY * DIRECTIONS - 15.0;


    return (color_blur + color_chab) / 2.;
}