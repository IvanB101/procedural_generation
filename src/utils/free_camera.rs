use bevy::prelude::*;

use crate::{camera::MainCamera, input_handling::KeyBindings, player::CameraHolder, AppState};

pub struct FreeCamera;

impl Plugin for FreeCamera {
    fn build(&self, app: &mut App) {
        app //
            .add_systems(Update, move_camera.run_if(in_state(AppState::InGame)));
    }
}

const BASE_SPEED: f32 = 5.;
const SPRINTING_FACTOR: f32 = 2.5;

fn move_camera(
    mut cameraholder_q: Query<&mut Transform, (With<CameraHolder>, Without<MainCamera>)>,
    fixed_time: Res<Time<Fixed>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    keybindings: Res<KeyBindings>,
    mut sprinting: Local<bool>,
) {
    let mut camera_transform = cameraholder_q.single_mut();

    let mut horizontal_input_vector = Vec3::ZERO;
    let local_z = camera_transform.local_z();
    let forward = -Vec3::new(local_z.x, 0., local_z.z).normalize_or_zero();
    let right = Vec3::new(local_z.z, 0., -local_z.x).normalize_or_zero();

    if let Some(forward_key) = keybindings.forward {
        if keyboard_input.pressed(forward_key) {
            horizontal_input_vector += forward;
        }
    }

    if let Some(back_key) = keybindings.back {
        if keyboard_input.pressed(back_key) {
            horizontal_input_vector += -forward;
        }
    }

    if let Some(left_key) = keybindings.left {
        if keyboard_input.pressed(left_key) {
            horizontal_input_vector += -right;
        }
    }

    if let Some(right_key) = keybindings.right {
        if keyboard_input.pressed(right_key) {
            horizontal_input_vector += right;
        }
    }

    let mut speed = BASE_SPEED;

    if let Some(sprint_key) = keybindings.sprint {
        if keyboard_input.just_pressed(sprint_key) {
            *sprinting = !*sprinting;
        }
    }

    if *sprinting {
        speed *= SPRINTING_FACTOR;
    }

    let horizontal_movement_translation = horizontal_input_vector.normalize_or_zero() * speed;

    camera_transform.translation += horizontal_movement_translation * fixed_time.delta_seconds();

    // vertical
    let mut vertical_input_vector = Vec3::ZERO;

    if let Some(jump_key) = keybindings.jump {
        if keyboard_input.pressed(jump_key) {
            vertical_input_vector += Vec3::Y;
        }
    }

    if let Some(jump_key) = keybindings.down {
        if keyboard_input.pressed(jump_key) {
            vertical_input_vector -= Vec3::Y;
        }
    }

    let vertical_movement_translation = vertical_input_vector.normalize_or_zero() * speed;

    camera_transform.translation += vertical_movement_translation * fixed_time.delta_seconds();
}
