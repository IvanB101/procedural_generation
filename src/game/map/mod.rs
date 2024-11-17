use bevy::prelude::*;
use height_map::HeightMap;

mod height_map;

#[derive(Component)]
pub struct Map;

#[derive(Resource, Clone, Copy, Debug)]
pub struct MapInfo {
    pub size: usize,
    pub unit_size: f32,
    pub min_depth: f32,
    pub max_depth: f32,
}

impl Default for MapInfo {
    fn default() -> Self {
        MapInfo {
            size: 1000,
            unit_size: 0.1_f32,
            min_depth: -0.2,
            max_depth: 5.,
        }
    }
}

impl Plugin for Map {
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
    let terrain_mesh = Mesh::from(HeightMap::new(
        map_info.size,
        map_info.unit_size,
        map_info.min_depth,
        map_info.max_depth,
    ));

    commands.spawn((PbrBundle {
        mesh: meshes.add(terrain_mesh),
        material: materials.add(StandardMaterial { ..default() }),
        ..default()
    },));
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
