use bevy::{math::FloatExt, prelude::Vec2};
use rand::prelude::*;

use super::Noise;
pub struct Perlin {
    permutation: Vec<usize>,
    wrap: usize,
    layers: Vec<(f32, f32)>,
}

impl Perlin {
    pub fn new(layers: &[(f32, f32)], wrap: usize, seed: Option<u64>) -> Self {
        let seed = seed.unwrap_or(0);
        let mut permutation: Vec<usize> = (0..wrap).collect();
        let mut rng = StdRng::seed_from_u64(seed);
        permutation.shuffle(&mut rng);

        permutation.append(&mut permutation.clone());

        println!("Len: {:?}", permutation.len());

        let infl_sum: f32 = layers.iter().map(|(weight, _)| weight).sum();
        let layers = layers
            .iter()
            .map(|(weight, compression_factor)| (weight / infl_sum, *compression_factor))
            .collect();

        Perlin {
            permutation,
            wrap,
            layers,
        }
    }
}

impl Noise for Perlin {
    type Input = (f32, f32);
    type Output = f32;

    fn get(&self, input: (f32, f32)) -> f32 {
        let mut value = 0.;

        for (weight, compression_factor) in &self.layers {
            let x = input.0 * compression_factor;
            let y = input.1 * compression_factor;
            let xf = x - x.floor();
            let yf = y - y.floor();
            let x = x as usize & (self.wrap - 1);
            let y = y as usize & (self.wrap - 1);

            let top_right = Vec2::new(xf - 1.0, yf - 1.0);
            let top_left = Vec2::new(xf, yf - 1.0);
            let bottom_right = Vec2::new(xf - 1.0, yf);
            let bottom_left = Vec2::new(xf, yf);

            let value_top_right = self.permutation[self.permutation[x + 1] + y + 1];
            let value_top_left = self.permutation[self.permutation[x] + y + 1];
            let value_bottom_right = self.permutation[self.permutation[x + 1] + y];
            let value_bottom_left = self.permutation[self.permutation[x] + y];

            let dot_top_right = top_right.dot(Perlin::get_constant_vector(value_top_right));
            let dot_top_left = top_left.dot(Perlin::get_constant_vector(value_top_left));
            let dot_bottom_right =
                bottom_right.dot(Perlin::get_constant_vector(value_bottom_right));
            let dot_bottom_left = bottom_left.dot(Perlin::get_constant_vector(value_bottom_left));

            let u = fade(xf);
            let v = fade(yf);

            value += dot_bottom_left
                .lerp(dot_top_left, v)
                .lerp(dot_bottom_right.lerp(dot_top_right, v), u)
                * weight;
        }

        // Normalized to [0, 1]
        (1_f32 + value) / 2_f32
    }
}

trait ConstantVector<I, O> {
    fn get_constant_vector(v: I) -> O;
}

impl ConstantVector<usize, Vec2> for Perlin {
    fn get_constant_vector(v: usize) -> Vec2 {
        match v & 3 {
            0 => Vec2::new(1., 1.),
            1 => Vec2::new(-1., 1.),
            2 => Vec2::new(-1., -1.),
            _ => Vec2::new(1., -1.),
        }
    }
}

pub fn fade(t: f32) -> f32 {
    return ((6_f32 * t - 15_f32) * t + 10_f32) * t * t * t;
}
