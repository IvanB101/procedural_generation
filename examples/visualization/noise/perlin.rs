use bevy::prelude::*;
use bevy_inspector_egui::{prelude::*, InspectorOptions};
use common::{noise_playground, VecWrapper, IMAGE_DIMENSIONS};
use procedural_generation::utils::noise::{
    perlin::{perlin_2d::Perlin2D, perlin_3d::Perlin3D},
    Noise,
};

#[path = "../../common/mod.rs"]
mod common;

#[derive(Reflect, Resource, InspectorOptions, Clone)]
#[reflect(Resource, InspectorOptions)]
struct Configuration {
    layers: Vec<(f32, f32)>,
    wrap: usize,
    seed: Option<u64>,
    #[inspector(min = 0.0, max = 1.0)]
    compress_factor: f32,
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            compress_factor: 0.05,
            layers: vec![(0.5, 1.), (0.25, 2.), (0.125, 4.), (0.075, 8.)],
            wrap: 256,
            seed: Some(0),
        }
    }
}

impl From<Configuration> for VecWrapper<u8> {
    fn from(config: Configuration) -> Self {
        let (image_width, image_height) = IMAGE_DIMENSIONS;

        let mut colors = Vec::new();

        // let noise = Perlin2D::new(&config.layers, config.wrap, config.seed).map(|value: f32| {
        //     let out: u8 = (value * 256.).floor() as u8;

        //     (out, out, out)
        // });

        let noise = Perlin3D::new(&config.layers, config.wrap, config.seed).map(|value: f32| {
            let out: u8 = (value * 256.).floor() as u8;

            (out, out, out)
        });

        let x_factor = config.wrap as f32 / image_width as f32 * config.compress_factor;
        let y_factor = config.wrap as f32 / image_height as f32 * config.compress_factor;

        for y in 0..image_height {
            for x in 0..image_width {
                let (r, g, b) = noise.get((x as f32 * x_factor, y as f32 * y_factor, 0.));

                colors.push(r);
                colors.push(g);
                colors.push(b);
                colors.push(255);
            }
        }

        VecWrapper { vec: colors }
    }
}

fn main() {
    noise_playground::<Configuration>();
}
