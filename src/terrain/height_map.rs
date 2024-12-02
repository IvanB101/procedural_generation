use crate::utils::noise::perlin::perlin_2d::Perlin2D;
use crate::utils::noise::Noise;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::{prelude::*, render::render_asset::RenderAssetUsages};

pub struct HeightMap {
    pub size: f32,
    pub samples: usize,
    unit_size: f32,
    pub height_map: Vec<Vec<f32>>,
}

impl HeightMap {
    pub fn new<T: Noise<Input = (f32, f32), Output = f32>>(
        size: f32,
        samples: usize,
        min_depth: f32,
        max_depth: f32,
        noise: T,
    ) -> Self {
        let mut height_map = Vec::with_capacity(samples);
        let conv_factor = 256. / samples as f32 / 30.;
        let unit_size = size / samples as f32;

        for i in 0..samples {
            let mut row = Vec::with_capacity(samples);
            for j in 0..samples {
                row.push(
                    min_depth
                        + noise.get((i as f32 * conv_factor, j as f32 * conv_factor))
                            * (max_depth - min_depth),
                );
            }
            height_map.push(row);
        }

        HeightMap {
            size,
            samples,
            unit_size,
            height_map,
        }
    }
}

impl Default for HeightMap {
    fn default() -> HeightMap {
        HeightMap::new(
            100.,
            1000,
            -0.2,
            5.,
            Perlin2D::new(
                &[(0.5, 1.), (0.25, 2.), (0.125, 4.), (0.075, 8.)],
                256,
                None,
            ),
        )
    }
}

impl From<HeightMap> for Mesh {
    fn from(value: HeightMap) -> Mesh {
        let mut indices = Vec::new();
        let mut vertices = Vec::new();
        let mut uvs = Vec::new();

        let HeightMap {
            size,
            samples,
            unit_size,
            height_map,
            ..
        } = value;

        for x in 0..samples {
            for z in 0..samples {
                vertices.push([
                    x as f32 * unit_size - size / 2.,
                    height_map[x][z],
                    z as f32 * unit_size - size / 2.,
                ]);
                uvs.push([x as f32 / samples as f32, z as f32 / samples as f32]);
            }
        }

        for x in 0..(samples - 1) {
            for z in 0..(samples - 1) {
                // Triangle 1
                indices.push((x * samples + z) as u32);
                indices.push((x * samples + z + 1) as u32);
                indices.push(((x + 1) * samples + z) as u32);
                // Triangle 2
                indices.push(((x + 1) * samples + z) as u32);
                indices.push((x * samples + z + 1) as u32);
                indices.push(((x + 1) * samples + z + 1) as u32);
            }
        }

        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
        .with_inserted_indices(bevy::render::mesh::Indices::U32(indices))
        .with_computed_normals()
    }
}
