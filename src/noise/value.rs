use rand::Rng;

use super::perlin::fade;

pub struct ValueNoise {
    values: Vec<Vec<f32>>,
}

impl ValueNoise {
    pub fn new(size: usize, n_layers: usize, influence_fall: f32, detail: usize) -> Self {
        let mut values = Vec::with_capacity(size);
        let mut layers = Vec::with_capacity(n_layers);
        let n = ((size / detail + 1) >> n_layers) + 1;
        let mut rng = rand::thread_rng();

        for i in 0..n_layers {
            let samples = n << n_layers - i + 1;
            let mut layer = Vec::with_capacity(samples);

            for _ in 0..samples {
                let mut row = Vec::with_capacity(samples);

                for _ in 0..samples {
                    row.push(rng.gen::<f32>());
                }
                layer.push(row);
            }

            layers.push(layer);
        }

        for i in 0..size {
            let mut row = Vec::with_capacity(size);
            let mut inf_fall = 1.;

            for j in 0..size {
                let mut value = 0.;

                for k in 0..n_layers {
                    let units_per_value = (n << (n_layers - k)) * detail;
                    value += fade(
                        (layers[k][i / units_per_value][j / units_per_value]
                            - layers[k][i / units_per_value][j / units_per_value + 1])
                            * (j % (n << k)) as f32,
                    ) * inf_fall;
                }
                row.push(value);

                inf_fall *= influence_fall;
            }
            values.push(row);
        }

        ValueNoise { values }
    }

    pub fn get(&self, x: usize, y: usize) -> f32 {
        self.values[x][y]
    }
}
