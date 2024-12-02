pub mod perlin_2d;
pub mod perlin_3d;

pub fn fade(t: f32) -> f32 {
    return ((6_f32 * t - 15_f32) * t + 10_f32) * t * t * t;
}
