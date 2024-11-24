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
use procedural_generation::utils::noise::{perlin::Perlin, Noise};

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
            compress_factor: 0.03,
            layers: vec![(0.5, 1.), (0.25, 2.), (0.125, 4.), (0.075, 8.)],
            wrap: 256,
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

impl From<(Configuration, Global)> for ImageWrapper {
    fn from(value: (Configuration, Global)) -> Self {
        let (config, global) = value;

        let Configuration {
            layers, wrap, seed, ..
        } = config;

        let noise = Perlin::new(&layers, wrap, seed);
        let factor = config.compress_factor;

        let mut colors = Vec::new();

        for y in 0..global.height {
            for x in 0..global.width {
                let value = (noise.get((x as f32 * factor, y as f32 * factor)) * 256.) as u8;

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
