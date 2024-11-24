#import bevy_pbr::{pbr_fragment::pbr_input_from_standard_material, pbr_functions::alpha_discard}
#import bevy_pbr::{forward_io::{VertexOutput, FragmentOutput}, pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing}}

// @vertex
// fn vertex() -> @builtin(position) vec4<f32> {
//     return vec4<f32>(0.0, 0.0, 0.0, 1.0);
// }

const GRASS = vec4f(vec3f(19., 109., 21.) / 255., 1.);
const DIRT = vec4f(vec3f(45., 25., 20.) / 255., 1.);

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> @location(0) vec4<f32> {
    // ? generate a PbrInput struct from the StandardMaterial bindings
    var pbr_input = pbr_input_from_standard_material(in, is_front);

    // ? we can optionally modify the input before lighting and alpha_discard is applied
    if pbr_input.N.y > 0.89 {
        pbr_input.material.base_color = GRASS;
        // pbr_input.material.perceptual_roughness = 0.;
    } else {
        pbr_input.material.base_color = DIRT;
    }
    // pbr_input.material.base_color = select(vec4f(0.), vec4f(1.), length(pbr_input.N) > 1.);

    // ? alpha discard
    pbr_input.material.base_color = alpha_discard(pbr_input.material, pbr_input.material.base_color);


    var color: vec4<f32>;

    // ? apply lighting
    color = apply_pbr_lighting(pbr_input);

    // ? we can optionally modify the lit color before post-processing is applied
    // color = vec4<f32>(vec4<u32>(color * f32(my_extended_material.quantize_steps))) / f32(my_extended_material.quantize_steps);

    // ? apply in-shader post processing (fog, alpha-premultiply, and also tonemapping, debanding if the camera is non-hdr)
    // ? note this does not include fullscreen postprocessing effects like bloom.
    color = main_pass_post_lighting_processing(pbr_input, color);

    // ? we can optionally modify the final result here
    // color = color * 2.0;

    // ! TEMP
    // let centered_uv = in.uv - vec2<f32>(0.5, 0.5);
    // let dist = length(centered_uv);
    // let radius = 0.1;

    // if dist < radius {
    //     color = vec4<f32>(0.0, 0.0, 0.0, 1.0);
    // } else {
    //     color = vec4<f32>(1.0, 1.0, 1.0, 1.0);
    // }
    // color = vec4<f32>(length(in.uv / 2.));
    // !

    return color;
}

// ? Simple
// @fragment
// fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
//     return vec4<f32>(select(0, 1, in.world_normal.y < 0.85));
// }

// ? Funny
// @fragment
// fn fragment(@location(1) world_normal: vec3<f32>) -> @location(0) vec4<f32> {
//     if length(world_normal) < 1 {
//         return vec4<f32>(0);
//     }
//     return vec4<f32>(1);
// }