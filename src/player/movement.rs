use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::input_handling::KeyBindings;

use super::{CameraHolder, ModelHolder, Player};

#[derive(Component)]
pub(super) struct MovementStats {
    pub(super) base_speed: f32,
    pub(super) sprinting: bool,
    pub(super) sprinting_factor: f32,
    pub(super) jump_height: f32,
}

#[derive(Component)]
pub(super) struct PlayerAcceleration(pub Vec3);

#[derive(Component)]
pub(super) struct PlayerVelocity(pub Vec3);

pub(super) fn move_player(
    fixed_time: Res<Time<Fixed>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    keybindings: Res<KeyBindings>,
    cameraholder_q: Query<
        &Transform,
        (With<CameraHolder>, (Without<Player>, Without<ModelHolder>)),
    >,
    // mut modelholder_q: Query<
    //     &mut Transform,
    //     (With<ModelHolder>, (Without<Player>, Without<CameraHolder>)),
    // >,
    mut player_q: Query<
        (
            &mut Transform,
            &mut MovementStats,
            &mut PlayerAcceleration,
            &mut PlayerVelocity,
        ),
        With<Player>,
    >,
    mut controller_q: Query<&mut KinematicCharacterController, With<Player>>,
    output_q: Query<&KinematicCharacterControllerOutput, With<Player>>,
    rapier_configuration: Res<RapierConfiguration>,
) {
    let camera_transform = cameraholder_q.single();

    let mut input_vector = Vec3::ZERO;
    let local_z = camera_transform.local_z();
    let forward = -Vec3::new(local_z.x, 0., local_z.z).normalize_or_zero();
    let right = Vec3::new(local_z.z, 0., -local_z.x).normalize_or_zero();

    if let Some(forward_key) = keybindings.forward {
        if keyboard_input.pressed(forward_key) {
            input_vector += forward;
        }
    }

    if let Some(back_key) = keybindings.back {
        if keyboard_input.pressed(back_key) {
            input_vector += -forward;
        }
    }

    if let Some(left_key) = keybindings.left {
        if keyboard_input.pressed(left_key) {
            input_vector += -right;
        }
    }

    if let Some(right_key) = keybindings.right {
        if keyboard_input.pressed(right_key) {
            input_vector += right;
        }
    }

    let (mut player_transform, mut movement_stats, mut player_acceleration, mut player_velocity) =
        player_q.single_mut();

    let mut speed = movement_stats.base_speed;

    if let Some(sprint_key) = keybindings.sprint {
        if keyboard_input.just_pressed(sprint_key) {
            movement_stats.sprinting = !movement_stats.sprinting;
        }
    }

    if movement_stats.sprinting {
        speed *= movement_stats.sprinting_factor;
    }

    let input_vector_normalized = input_vector.normalize_or_zero();

    // Rotate model by movement dir
    if input_vector_normalized.is_normalized() {
        player_transform.look_to(input_vector_normalized, Vec3::Y);
    }

    let mut movement_translation = input_vector_normalized * speed;

    let mut controller = controller_q.single_mut();

    if let Ok(output) = output_q.get_single() {
        if output.grounded {
            player_acceleration.0 = Vec3::ZERO;
            player_velocity.0 = Vec3::ZERO;

            if let Some(jump_key) = keybindings.jump {
                if keyboard_input.pressed(jump_key) {
                    // movement_stats.jump_velocity =
                    //     f32::sqrt(movement_stats.jump_height * -3. * rapier_configuration.gravity.y);
                    // player_translation.y +=
                    //     movement_stats.jump_height / 2. / fixed_time.delta_seconds();

                    player_velocity.0.y +=
                        (2. * (-rapier_configuration.gravity.y) * movement_stats.jump_height)
                            .sqrt();
                }
            }
        } else {
            // Add gravity acceleration to player if not grounded
            player_acceleration.0 = rapier_configuration.gravity;
        }

        //Calculate velocity
        player_velocity.0 += player_acceleration.0 * fixed_time.delta_seconds();

        movement_translation += player_velocity.0;
    }

    controller.translation = Some(movement_translation * fixed_time.delta_seconds());

    // use to rotate?  output.desired_translation // output.effective_translation
}

// ? OLD
// fn movement(
//     keys: Res<ButtonInput<KeyCode>>,
//     mut query: Query<&mut Transform, With<FpsCam>>,
//     config: Res<CameraConfig>,
//     time: Res<Time>,
// ) {
//     let mut transform = query.single_mut();
//     let Vec3 { x, z, .. } = transform.forward().into();
//     let forward = Vec3::new(x, 0., z).normalize();
//     let right = Vec3::new(-z, 0., x).normalize();
//     let mut displacement = Vec3::new(0., 0., 0.);

//     for key in keys.get_pressed() {
//         match *key {
//             x if Some(x) == config.key_bindings.forward => displacement += forward,
//             x if Some(x) == config.key_bindings.back => displacement -= forward,
//             x if Some(x) == config.key_bindings.left => displacement -= right,
//             x if Some(x) == config.key_bindings.right => displacement += right,
//             x if Some(x) == config.key_bindings.up => displacement += Vec3::Y,
//             x if Some(x) == config.key_bindings.down => displacement -= Vec3::Y,
//             _ => (),
//         }
//     }

//     transform.translation +=
//         displacement.normalize_or_zero() * config.movespeed * time.delta_seconds();
// }
