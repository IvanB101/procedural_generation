#![allow(dead_code)]

#[allow(unused_imports)]
#[cfg(debug_assertions)]
use bevy_dylib;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier3d::prelude::*;

use camera::MyCameraPlugin;
use common::CommonPlugin;
use hud::HUDPlugin;
use input_handling::InputHandlingPlugin;
use player::PlayerPlugin;
use terrain::TerrainPlugin;

mod camera;
mod common;
mod compute_shader;
mod hud;
mod input_handling;
mod player;
mod post_processing;
mod terrain;
mod ui;
mod utils;

#[derive(Default, States, Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    MainMenu,
    #[default]
    InGame,
}

fn main() {
    App::new()
        // Default Plugin
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Procedural Generation".into(),
                visible: false, // false to make visible on Startup. Prevents long white window on start
                present_mode: bevy::window::PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        // Limit FPS and fix latency
        .add_plugins(bevy_framepace::FramepacePlugin)
        // State
        .init_state::<AppState>()
        // Resources
        // Rapier
        .insert_resource({
            let mut rap_conf = RapierConfiguration::new(1.);
            rap_conf.timestep_mode = TimestepMode::Fixed {
                dt: 1. / 64.,
                substeps: 1,
            };
            rap_conf
        })
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default().in_fixed_schedule())
        // Comment DebugRenderer to disable
        // .add_plugins(RapierDebugRenderPlugin::default())
        // Plugins
        .add_plugins((
            CommonPlugin,
            MyCameraPlugin,
            HUDPlugin,
            PlayerPlugin,
            InputHandlingPlugin,
            TerrainPlugin,
            // PostProcessPlugin,
            // GameOfLifeComputePlugin,
        ))
        // Systems
        .add_systems(Startup, (make_visible, setup))
        .run();
}

// Make visible on Startup. Prevents long white window on start
fn make_visible(mut window: Query<&mut Window, With<PrimaryWindow>>) {
    window.single_mut().visible = true;
}

fn setup(
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
    // mut rapier_config: ResMut<RapierConfiguration>,
    // time_fixed: Res<Time<Fixed>>,
) {
    // // Rapier TimeStep
    // rapier_config.timestep_mode = TimestepMode::Fixed {
    //     dt: time_fixed.timestep().as_secs_f32(),
    //     substeps: 1,
    // };

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 10.,
        ..default()
    });

    // commands.spawn(PointLightBundle {
    //     transform: Transform::from_xyz(0., 0., 0.),
    //     point_light: PointLight {
    //         intensity: 100_000.0,
    //         color: Color::WHITE,
    //         shadows_enabled: true,
    //         ..default()
    //     },
    //     ..default()
    // });

    // directional 'sun' light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 500., // light_consts::lux::CLEAR_SUNRISE
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::PI / 4.),
            ..default()
        },
        // // The default cascade config is designed to handle large scenes.
        // // As this example has a much smaller world, we can tighten the shadow
        // // bounds for better visual quality.
        // cascade_shadow_config: CascadeShadowConfigBuilder {
        //     first_cascade_far_bound: 4.0,
        //     maximum_distance: 10.0,
        //     ..default()
        // }
        // .into(),
        ..default()
    });

    // // ground plane
    // commands
    //     .spawn(PbrBundle {
    //         transform: Transform::from_xyz(0., 0., 0.),
    //         mesh: meshes.add(Plane3d::default().mesh().size(10.0, 10.0)),
    //         material: materials.add(StandardMaterial {
    //             base_color: Srgba::gray(0.25).into(),
    //             perceptual_roughness: 1.0,
    //             ..default()
    //         }),
    //         ..default()
    //     })
    //     .with_children(|children| {
    //         children
    //             .spawn(Collider::cuboid(50.0, 1., 50.0))
    //             // .insert(Restitution::coefficient(0.5))
    //             .insert(TransformBundle::from(Transform::from_xyz(0.0, -1.0, 0.0)));
    //     });
}
