use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use height_map::HeightMap;

use crate::utils::noise::perlin::Perlin;

mod height_map;

pub struct TerrainPlugin;

#[derive(Resource, Clone, Copy, Debug)]
pub struct MapInfo {
    pub size: f32,
    pub samples: usize,
    pub min_depth: f32,
    pub max_depth: f32,
}

impl Default for MapInfo {
    fn default() -> Self {
        MapInfo {
            size: 50.,
            samples: 1000,
            min_depth: -2.5,
            max_depth: 2.5,
        }
    }
}

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MapInfo>()
            // .add_systems(Startup, setup)
            .add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    map_info: Res<MapInfo>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let height_map = HeightMap::new(
        map_info.size,
        map_info.samples,
        map_info.min_depth,
        map_info.max_depth,
        Perlin::new(
            &[(0.5, 1.), (0.25, 2.), (0.125, 4.), (0.075, 8.)],
            256,
            None,
        ),
    );
    let height_collider = Collider::heightfield(
        height_map.height_map.concat(),
        height_map.samples,
        height_map.samples,
        Vec3::new(map_info.size, 1., map_info.size),
    );
    let terrain_mesh = Mesh::from(height_map);
    // let mesh_collider = Collider::from_bevy_mesh(&terrain_mesh, &ComputedColliderShape::TriMesh)
    //     .expect("es una pija");

    commands
        .spawn(PbrBundle {
            transform: Transform::from_xyz(0., 0., 0.),
            mesh: meshes.add(terrain_mesh),
            material: materials.add(StandardMaterial { ..default() }),
            ..default()
        })
        .with_children(|children| {
            children
                .spawn(height_collider)
                // .insert(Restitution::coefficient(0.5))
                .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)));
        });
}

// fn render_map(
//     mut commands: Commands,
//     map_info: Res<MapInfo>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
//     mut meshes: ResMut<Assets<Mesh>>,
// ) {
//     if !map_info.is_changed() {
//         return;
//     }
// }
