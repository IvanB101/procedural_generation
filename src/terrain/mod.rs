use bevy::{
    pbr::{ExtendedMaterial, MaterialExtension, OpaqueRendererMethod},
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};
// use bevy_rapier3d::prelude::*;

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
            min_depth: -3.,
            max_depth: 3.,
        }
    }
}

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MapInfo>()
            .add_plugins(MaterialPlugin::<
                ExtendedMaterial<StandardMaterial, MyCustomExtension>,
            >::default())
            .add_systems(Startup, setup)
            // .add_systems(Update, change_uniform)
            ;
    }
}

fn setup(
    mut commands: Commands,
    map_info: Res<MapInfo>,
    // mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, MyExtension>>>,
    mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, MyCustomExtension>>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let height_map = HeightMap::new(
        map_info.size,
        map_info.samples,
        map_info.min_depth,
        map_info.max_depth,
        Perlin::new(
            &[(0.75, 1.), (0.25, 2.)],
            // &[(0.5, 1.), (0.25, 2.), (0.125, 4.), (0.075, 8.)],
            256,
            None,
        ),
    );
    // let height_collider = Collider::heightfield(
    //     height_map.height_map.concat(),
    //     height_map.samples,
    //     height_map.samples,
    //     Vec3::new(map_info.size, 1., map_info.size),
    // );
    let terrain_mesh = Mesh::from(height_map);
    // let mesh_collider = Collider::from_bevy_mesh(&terrain_mesh, &ComputedColliderShape::TriMesh)
    //     .expect("es una pija");

    commands.spawn((
        Transform::from_xyz(0., 0., 0.),
        Mesh3d(meshes.add(terrain_mesh)),
        MeshMaterial3d(materials.add(ExtendedMaterial {
            base: StandardMaterial {
                // base_color: RED.into(),
                // can be used in forward or deferred mode.
                opaque_render_method: OpaqueRendererMethod::Forward,
                // in deferred mode, only the PbrInput can be modified (uvs, color and other material properties),
                // in forward mode, the output can also be modified after lighting is applied.
                // see the fragment shader `extended_material.wgsl` for more info.
                // Note: to run in deferred mode, you must also add a `DeferredPrepass` component to the camera and either
                // change the above to `OpaqueRendererMethod::Deferred` or add the `DefaultOpaqueRendererMethod` resource.
                // ? weird alpha
                // alpha_mode: AlphaMode::Blend,
                perceptual_roughness: 1.,
                ..Default::default()
            },
            extension: MyCustomExtension {
                quantize_steps: 100,
            },
        })),
    ));

    // commands
    //     .spawn(MaterialMeshBundle {
    //         transform: Transform::from_xyz(0., 0., 0.),
    //         mesh: meshes.add(terrain_mesh),
    //         material: materials.add(ExtendedMaterial {
    //             base: StandardMaterial {
    //                 // base_color: RED.into(),
    //                 // can be used in forward or deferred mode.
    //                 opaque_render_method: OpaqueRendererMethod::Forward,
    //                 // in deferred mode, only the PbrInput can be modified (uvs, color and other material properties),
    //                 // in forward mode, the output can also be modified after lighting is applied.
    //                 // see the fragment shader `extended_material.wgsl` for more info.
    //                 // Note: to run in deferred mode, you must also add a `DeferredPrepass` component to the camera and either
    //                 // change the above to `OpaqueRendererMethod::Deferred` or add the `DefaultOpaqueRendererMethod` resource.
    //                 // ? weird alpha
    //                 // alpha_mode: AlphaMode::Blend,
    //                 perceptual_roughness: 1.,
    //                 ..Default::default()
    //             },
    //             extension: MyCustomExtension {
    //                 quantize_steps: 100,
    //             },
    //         }),
    //         ..default()
    //     })
    //     .with_children(|children| {
    //         children
    //             .spawn(height_collider)
    //             // .insert(Restitution::coefficient(0.5))
    //             .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)));
    //     });
}

// fn change_uniform(
//     q_my_custom_extension: Query<&Handle<ExtendedMaterial<StandardMaterial, MyCustomExtension>>>,
//     mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, MyCustomExtension>>>,
// ) {
//     for handle in q_my_custom_extension.iter() {
//         // info!("{:?}", h);
//         let mat = materials
//             .get_mut(handle)
//             .expect("ExtendedMaterial not in assets.");
//         // info!("{:?}", mat.extension.log);
//         mat.extension.log = 1.;
//     }
// }

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
struct MyCustomExtension {
    // We need to ensure that the bindings of the base material and the extension do not conflict,
    // so we start from binding slot 100, leaving slots 0-99 for the base material.
    #[uniform(100)]
    quantize_steps: u32,
}

impl MaterialExtension for MyCustomExtension {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_extension.wgsl".into()
        // "shaders/extended_material.wgsl".into()
    }

    // fn deferred_fragment_shader() -> ShaderRef {
    //     "shaders/custom_extension.wgsl".into()
    // }

    // fn vertex_shader() -> ShaderRef {
    //     "shaders/custom_extension.wgsl".into()
    // }
}
