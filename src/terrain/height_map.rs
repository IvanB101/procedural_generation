use crate::utils::noise::{perlin::Perlin, Noise};
use bevy::render::render_resource::PrimitiveTopology;
use bevy::{prelude::*, render::render_asset::RenderAssetUsages};

pub struct HeightMap {
    pub size: f32,
    pub samples: usize,
    unit_size: f32,
    pub height_map: Vec<Vec<f32>>,
}

impl HeightMap {
    pub fn new(size: f32, samples: usize, min_depth: f32, max_depth: f32) -> Self {
        let mut height_map = Vec::with_capacity(samples + 1);
        let wrap = 256;
        let cord_to_wrap = wrap / 4;
        let noise = Perlin::new(Vec::from([0.5, 0.25, 0.125, 0.075]), wrap);

        for i in 0..samples {
            let mut row = Vec::with_capacity(samples + 1);
            for j in 0..samples {
                row.push(
                    min_depth
                        + noise.get(&[
                            i as f32 / cord_to_wrap as f32,
                            j as f32 / cord_to_wrap as f32,
                        ]) * (max_depth - min_depth),
                );
            }
            height_map.push(row);
        }

        HeightMap {
            size,
            samples,
            unit_size: size / samples as f32,
            height_map,
        }
    }
}

impl Default for HeightMap {
    fn default() -> HeightMap {
        HeightMap::new(100., 1000, -0.2, 5.)
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
