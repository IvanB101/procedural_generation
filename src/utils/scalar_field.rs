use crate::utils::noise::Noise;

pub struct ScalarField2D<T> {
    pub values: Vec<T>,
    pub dimensions: (u32, u32),
    pub unit_size: f32,
    pub surface: f32,
}

pub struct ScalarField3D<T> {
    pub values: Vec<T>,
    pub dimensions: (u32, u32, u32),
    pub unit_size: f32,
    pub surface: f32,
}

impl<T> ScalarField3D<T> {
    pub fn new(values: Vec<T>, dimensions: (u32, u32, u32), unit_size: f32, surface: f32) -> Self {
        Self {
            values,
            dimensions,
            unit_size,
            surface,
        }
    }

    pub fn fill<N: Noise<Input = (f32, f32, f32), Output = T>>(
        dimensions: (u32, u32, u32),
        unit_size: f32,
        surface: f32,
        noise: N,
    ) -> Self {
        let mut values = Vec::new();

        // TODO
        let comp_factor = 0.03;

        for z in 0..dimensions.2 as usize {
            for y in 0..dimensions.1 as usize {
                for x in 0..dimensions.0 as usize {
                    values.push(noise.get((
                        x as f32 * comp_factor,
                        y as f32 * comp_factor,
                        z as f32 * comp_factor,
                    )));
                }
            }
        }

        Self {
            values,
            dimensions,
            unit_size,
            surface,
        }
    }
}
