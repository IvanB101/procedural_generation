use bevy::math::FloatExt;
use rand::prelude::*;

use crate::utils::noise::Noise;

use super::fade;

#[path = "../../scalar_field.rs"]
mod scalar_field;

pub struct Perlin3D {
    permutation: Vec<usize>,
    wrap: usize,
    layers: Vec<(f32, f32)>,
}

impl Perlin3D {
    pub fn new(layers: &[(f32, f32)], wrap: usize, seed: Option<u64>) -> Self {
        let seed = seed.unwrap_or(0);
        let mut permutation: Vec<usize> = (0..wrap).collect();
        let mut rng = StdRng::seed_from_u64(seed);
        permutation.shuffle(&mut rng);

        let mut clone = &mut permutation.clone();
        permutation.append(&mut clone.clone());
        permutation.append(&mut clone);

        let infl_sum: f32 = layers.iter().map(|(weight, _)| weight).sum();
        let layers = layers
            .iter()
            .map(|(weight, compression_factor)| (weight / infl_sum, *compression_factor))
            .collect();

        Perlin3D {
            permutation,
            wrap,
            layers,
        }
    }
}

impl Noise for Perlin3D {
    type Input = (f32, f32, f32);
    type Output = f32;

    fn get(&self, input: Self::Input) -> Self::Output {
        let mut value = 0.;
        let p = &self.permutation;

        for (weight, compression_factor) in &self.layers {
            let x = input.0 * compression_factor;
            let y = input.1 * compression_factor;
            let z = input.2 * compression_factor;
            let xf = x - x.floor();
            let yf = y - y.floor();
            let zf = z - z.floor();
            let xi = x as usize & (self.wrap - 1);
            let yi = y as usize & (self.wrap - 1);
            let zi = z as usize & (self.wrap - 1);

            let aaa = p[p[p[xi] + yi] + zi];
            let aba = p[p[p[xi] + yi + 1] + zi];
            let aab = p[p[p[xi] + yi] + zi + 1];
            let abb = p[p[p[xi] + yi + 1] + zi + 1];
            let baa = p[p[p[xi + 1] + yi] + zi];
            let bba = p[p[p[xi + 1] + yi + 1] + zi];
            let bab = p[p[p[xi + 1] + yi] + zi + 1];
            let bbb = p[p[p[xi + 1] + yi + 1] + zi + 1];

            let u = fade(xf);
            let v = fade(yf);
            let w = fade(zf);

            let x1 = grad(aaa, xf, yf, zf).lerp(grad(baa, xf - 1., yf, zf), u);
            let x2 = grad(aba, xf, yf - 1., zf).lerp(grad(bba, xf - 1., yf - 1., zf), u);
            let y1 = x1.lerp(x2, v);

            let x1 = grad(aab, xf, yf, zf - 1.).lerp(grad(bab, xf - 1., yf, zf - 1.), u);
            let x2 = grad(abb, xf, yf - 1., zf - 1.).lerp(grad(bbb, xf - 1., yf - 1., zf - 1.), u);
            let y2 = x1.lerp(x2, v);

            value += y1.lerp(y2, w) * weight;
        }

        // Normalized to [0, 1]
        (1. + value) / 2.
    }
}

fn grad(v: usize, x: f32, y: f32, z: f32) -> f32 {
    match v & 0xf {
        0x0 => x + y,
        0x1 => -x + y,
        0x2 => x - y,
        0x3 => -x - y,
        0x4 => x + z,
        0x5 => -x + z,
        0x6 => x - z,
        0x7 => -x - z,
        0x8 => y + z,
        0x9 => -y + z,
        0xA => y - z,
        0xB => -y - z,
        0xC => y + x,
        0xD => -y + z,
        0xE => y - x,
        0xF => -y - z,
        _ => 0.,
    }
}
