use bevy::{prelude::*, window::PrimaryWindow};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use procedural_generation::{
    camera::MyCameraPlugin,
    common::CommonPlugin,
    input_handling::InputHandlingPlugin,
    utils::{
        free_camera::FreeCamera, noise::perlin::perlin_3d::Perlin3D, scalar_field::ScalarField3D,
    },
    AppState,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Marching cubes".into(),
                visible: false, // false to make visible on Startup. Prevents long white window on start
                present_mode: bevy::window::PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        // Limit FPS and fix latency
        .add_plugins(bevy_framepace::FramepacePlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .init_state::<AppState>()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins((
            CommonPlugin,
            FreeCamera,
            MyCameraPlugin,
            InputHandlingPlugin,
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    // mut rapier_config: ResMut<RapierConfiguration>,
    // time_fixed: Res<Time<Fixed>>,
) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 10.,
        ..default()
    });

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
        ..default()
    });

    let mesh: Mesh = ScalarField3D::fill(
        (100, 100, 100),
        1.,
        0.5,
        Perlin3D::new(
            &[(0.5, 1.), (0.25, 2.), (0.125, 4.), (0.075, 8.)],
            256,
            Some(0),
        ),
    )
    .into();
    let material = materials.add(StandardMaterial {
        base_color: Srgba::gray(0.25).into(),
        perceptual_roughness: 1.0,
        ..default()
    });

    commands.spawn(PbrBundle {
        transform: Transform::from_xyz(0., 0., 0.),
        mesh: meshes.add(mesh),
        material: material.clone(),
        ..default()
    });

    commands.spawn(PbrBundle {
        transform: Transform::from_xyz(0., 0., 0.),
        mesh: meshes.add(Plane3d::default().mesh().size(10.0, 10.0)),
        material,
        ..default()
    });
}
