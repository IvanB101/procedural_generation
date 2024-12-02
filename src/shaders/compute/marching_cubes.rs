use bevy::{
    input::ButtonInput,
    prelude::{MouseButton, Res, ResMut, Resource, World},
    reflect::TypePath,
};
use bevy_easy_compute::prelude::{
    AppComputeWorker, AppComputeWorkerBuilder, ComputeShader, ComputeWorker, ShaderRef,
};
use rand::prelude::*;
use rand::rngs::StdRng;

const DIMENSIONS: (u32, u32, u32) = (500, 500, 500);
const GROUP_SIZE: u32 = 8;

struct Settings {
    permutation: Vec<u32>,
    wrap: u32,
    seed: u64,
    layers: Vec<(f32, f32)>,
}

impl Default for Settings {
    fn default() -> Self {
        let wrap = 256;
        let mut permutation: Vec<u32> = (0..wrap).collect();
        let mut rng = StdRng::seed_from_u64(0);
        permutation.shuffle(&mut rng);
        permutation.append(&mut permutation.clone());

        Self {
            layers: vec![(0.5, 1.), (0.25, 2.), (0.125, 4.), (0.075, 8.)],
            permutation,
            wrap,
            seed: 0,
        }
    }
}

#[derive(TypePath)]
struct MarchingCubesShader;

impl ComputeShader for MarchingCubesShader {
    fn shader() -> ShaderRef {
        "shaders/compute/marching_cubes.wgsl".into()
    }
}
#[derive(Resource)]
struct MarchingCubesComputeWorker;

impl ComputeWorker for MarchingCubesComputeWorker {
    fn build(world: &mut World) -> AppComputeWorker<Self> {
        let Settings {
            layers,
            permutation,
            wrap,
            ..
        } = Settings::default();

        let worker = AppComputeWorkerBuilder::new(world)
            .add_uniform("wrap", &wrap)
            .add_uniform("layer_num", &wrap)
            .add_staging("permutation", &permutation)
            .add_staging(
                "layers",
                &layers.iter().fold(Vec::new(), |mut arr, (a, b)| {
                    arr.push(a);
                    arr.push(b);
                    arr
                }),
            )
            .add_staging(
                "vertices",
                &Vec::<f32>::with_capacity(
                    (DIMENSIONS.0 * DIMENSIONS.1 * DIMENSIONS.2 * 3) as usize,
                ),
            )
            .add_pass::<MarchingCubesShader>(
                [
                    DIMENSIONS.0 / GROUP_SIZE,
                    DIMENSIONS.1 / GROUP_SIZE,
                    DIMENSIONS.2 / GROUP_SIZE,
                ],
                &["uni", "values"],
            )
            .one_shot()
            .build();

        worker
    }
}

fn my_system(mut compute_worker: ResMut<AppComputeWorker<MarchingCubesComputeWorker>>) {
    if !compute_worker.ready() {
        return;
    }

    compute_worker.write("uni", &0.);

    // let vertices: Vec<f32> = compute_worker.read_vec("values");
}

fn update(
    buttons: Res<ButtonInput<MouseButton>>,
    mut compute_worker: ResMut<AppComputeWorker<MarchingCubesComputeWorker>>,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }

    compute_worker.execute();
}
