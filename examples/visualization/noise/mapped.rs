use bevy::prelude::*;
use bevy_inspector_egui::{prelude::*, InspectorOptions};
use common::{noise_playground, VecWrapper, IMAGE_DIMENSIONS};
use procedural_generation::utils::noise::{cellular::Cellular, Noise};

#[path = "../../common/mod.rs"]
mod common;

#[derive(Reflect, Resource, InspectorOptions, Clone)]
#[reflect(Resource, InspectorOptions)]
struct Configuration {
    width: u64,
    height: u64,
    seed: Option<u64>,
    // #[inspector(min = 0.0, max = 1.0)]
}

impl Default for Configuration {
    fn default() -> Self {
        let width = 30;
        let height = 20;

        Configuration {
            width,
            height,
            seed: Some(0),
        }
    }
}

impl From<Configuration> for VecWrapper<u8> {
    fn from(config: Configuration) -> Self {
        let (image_width, image_height) = IMAGE_DIMENSIONS;

        let mut colors = Vec::new();

        let noise = Cellular::new(config.width, config.height, config.seed).map(|input: f32| {
            let value = ((1. - input).sqrt() - 0.2).clamp(0., 1.);

            let out = (value * 256.).floor() as u8;

            (out, out, out)
        });

        let x_factor = config.width as f32 / image_width as f32;
        let y_factor = config.height as f32 / image_height as f32;

        for y in 0..image_height {
            for x in 0..image_width {
                let (r, g, b) = noise.get((x as f32 * x_factor, y as f32 * y_factor));

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
