use bevy::{color::palettes::css::FUCHSIA, prelude::*};
use bevy_rapier3d::prelude::*;

use movement::{MovementStats, PlayerAcceleration, PlayerVelocity};

use crate::AppState;

mod movement;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    movement: MovementStats,
    velocity: PlayerVelocity,
    acceleration: PlayerAcceleration,
    // weapon: Weapon,
    player_mark: Player,
}

#[derive(Component)]
pub struct ModelHolder;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app //
            .add_systems(Startup, setup_player)
            .add_systems(
                Update,
                (movement::move_player).run_if(in_state(AppState::InGame)),
            );
    }
}

fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    const HEIGHT: f32 = 1.5;

    commands
        .spawn(PlayerBundle {
            movement: MovementStats {
                base_speed: 3.,
                sprinting: false,
                sprinting_factor: 1.5,
                jump_height: 1.25,
            },
            acceleration: PlayerAcceleration(Vec3::ZERO),
            velocity: PlayerVelocity(Vec3::ZERO),
            // weapon: Weapon::new(20., 10.),
            player_mark: Player {},
        })
        .insert(SpatialBundle::from_transform(Transform::from_xyz(
            0., 8., 0.,
        )))
        .insert(RigidBody::KinematicPositionBased)
        .insert({
            let radius = HEIGHT / 5.5;
            Collider::capsule_y(HEIGHT / 2. - radius, radius)
        })
        // .insert(Collider::cuboid(HEIGHT / 5., HEIGHT / 2., HEIGHT / 8.))
        .insert(LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z)
        .insert(KinematicCharacterController {
            // custom_shape: Some((
            //     Collider::cuboid(HEIGHT / 4., HEIGHT / 2., HEIGHT / 8.),
            //     Vec3::new(0., 0., 0.),
            //     Quat::IDENTITY,
            // )),
            ..KinematicCharacterController::default()
        })
        .with_children(|parent| {
            // 3D Model
            parent
                .spawn(SpatialBundle::from_transform(Transform::from_xyz(
                    0., 0., 0.,
                )))
                .insert(ModelHolder)
                .with_children(|parent| {
                    // Head
                    parent.spawn(PbrBundle {
                        mesh: meshes.add(Mesh::from(Cuboid::from_size(Vec3::new(
                            HEIGHT / 5.,
                            HEIGHT * 0.25,
                            HEIGHT / 5.,
                        )))),
                        material: materials.add(Color::from(FUCHSIA)),
                        transform: Transform::from_xyz(0., HEIGHT * 0.375, 0.),
                        ..default()
                    });
                    // Body
                    parent.spawn(PbrBundle {
                        mesh: meshes.add(Mesh::from(Cuboid::from_size(Vec3::new(
                            HEIGHT / 2.5,
                            HEIGHT * 0.75,
                            HEIGHT / 4.,
                        )))),
                        material: materials.add(Color::from(FUCHSIA)),
                        transform: Transform::from_xyz(0., -HEIGHT * 0.125, 0.),
                        ..default()
                    });
                });

            // Camera as child of player (not working)
            // parent
            //     .spawn(SpatialBundle::from_transform(Transform::from_xyz(
            //         0., 0.6, 0.,
            //     )))
            //     .insert(CameraHolder)
            //     .with_children(|parent| {
            //         parent
            //             .spawn(Camera3dBundle {
            //                 transform: Transform::from_xyz(0., 0., CAMERA_DISTANCE)
            //                     .looking_at(Vec3::ZERO, Vec3::Y),
            //                 ..default()
            //             })
            //             .insert(MainCamera);
            //     });
        });
    // .insert(ExternalForce {
    //     force: Vec3::new(10.0, 20.0, 30.0),
    //     torque: Vec3::new(1.0, 2.0, 3.0),
    // })
    // .insert(ExternalImpulse {
    //     impulse: Vec3::new(0.0, 1.0, -1.0),
    //     torque_impulse: Vec3::new(0.1, 0.2, 0.3),
    // });

    // ? Test Dummy
    // commands
    //     .spawn(PbrBundle {
    //         mesh: meshes.add(Mesh::from(Cuboid::from_size(Vec3::new(
    //             HEIGHT / 2.5,
    //             HEIGHT,
    //             HEIGHT / 4.,
    //         )))),
    //         material: materials.add(Color::from(FUCHSIA)),
    //         transform: Transform::from_xyz(1., 10., 12.),
    //         ..default()
    //     })
    //     .insert(RigidBody::Dynamic)
    //     .insert(Collider::cuboid(HEIGHT / 5., HEIGHT / 2., HEIGHT / 8.))
    //     .insert(LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z);
}
