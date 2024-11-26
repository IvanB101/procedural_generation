use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
};
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::InspectorOptions;
use common::{noise_playground, ImageWrapper, TransformWrapper};
use procedural_generation::utils::noise::{cellular::Cellular, mapper::Mapper, Noise};

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

#[derive(Resource, Clone)]
struct Global {
    width: u32,
    height: u32,
}

impl Default for Global {
    fn default() -> Self {
        Global {
            width: 1920,
            height: 1080,
        }
    }
}

fn mapper(value: f32) -> f32 {
    let threshold = 0.05;

    if value < threshold {
        1. + value * (1. - 1. / threshold)
    } else {
        value
    }
}

// f(0) = 1
// f(t) = t
// m = (t - 1) / (t - 0)

impl From<(Configuration, Global)> for ImageWrapper {
    fn from(value: (Configuration, Global)) -> Self {
        let (config, global) = value;

        let Configuration {
            width,
            height,
            seed,
            ..
        } = config;

        let mut noise = Mapper::<Cellular<(f32, f32)>>::new(Cellular::new(width, height, seed));
        noise.add_custom(mapper);

        let mut colors = Vec::new();

        for y in 0..global.height {
            for x in 0..global.width {
                let value = (noise.get((
                    x as f32 * (config.width as f32 / global.width as f32),
                    y as f32 * (config.height as f32 / global.height as f32),
                )) * 256.) as u8;

                colors.push(value);
                colors.push(value);
                colors.push(value);
                colors.push(255);
            }
        }

        ImageWrapper {
            image: Image::new_fill(
                Extent3d {
                    width: global.width,
                    height: global.height,
                    depth_or_array_layers: 1,
                },
                TextureDimension::D2,
                &colors,
                TextureFormat::Rgba8Unorm,
                RenderAssetUsages::RENDER_WORLD,
            ),
        }
    }
}

impl From<(Configuration, Global)> for TransformWrapper {
    fn from(value: (Configuration, Global)) -> Self {
        let (_conf, global) = value;
        TransformWrapper {
            transform: Transform::default().with_scale(Vec3 {
                x: global.width as f32,
                y: global.height as f32,
                z: 1.,
            }),
        }
    }
}

fn main() {
    noise_playground::<Configuration, Global>();
}
