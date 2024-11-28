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
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            width: 30,
            height: 20,
            seed: Some(0),
        }
    }
}

impl From<Configuration> for VecWrapper<u8> {
    fn from(config: Configuration) -> Self {
        let (image_width, image_height) = IMAGE_DIMENSIONS;

        let mut colors = Vec::new();

        let noise = Cellular::new(config.width, config.height, config.seed).map(|value: f32| {
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
