use std::f32::consts::SQRT_2;

use rand::prelude::*;
use rand::rngs::StdRng;

use super::Noise;

pub struct Cellular<T> {
    width: u64,
    height: u64,
    points: Vec<T>,
}

impl Cellular<(f32, f32)> {
    pub fn new(width: u64, height: u64, seed: Option<u64>) -> Self {
        let seed = seed.unwrap_or(0);

        let mut rng = StdRng::seed_from_u64(seed);

        let points = (0..((width + 2) * (height + 2)))
            .map(|_| (rng.gen::<f32>(), rng.gen::<f32>()))
            .collect();

        Cellular {
            width,
            height,
            points,
        }
    }
}

impl Noise for Cellular<(f32, f32)> {
    type Input = (f32, f32);
    type Output = f32;

    fn get(&self, input: (f32, f32)) -> f32 {
        let mut min_x = 2.;
        let mut min_y = 2.;

        let in_x = input.0.floor() as u64 % self.width;
        let in_y = input.1.floor() as u64 % self.height;

        let curr_x = input.0 - input.0.floor();
        let curr_y = input.1 - input.1.floor();

        for x in 0..=2 {
            for y in 0..=2 {
                let (ox, oy) = self.points[((in_y + y) * (self.width + 2) + in_x + x) as usize];

                let dist_x = x as f32 + ox - curr_x - 1.;
                let dist_y = y as f32 + oy - curr_y - 1.;

                if dist_x * dist_x + dist_y * dist_y < min_x + min_y {
                    min_x = dist_x * dist_x;
                    min_y = dist_y * dist_y;
                }
            }
        }

        (min_x + min_y).sqrt() / SQRT_2
    }
}
