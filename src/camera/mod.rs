use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
// use bevy_rapier3d::{plugin::RapierContext, prelude::QueryFilter};

use crate::AppState;

pub struct MyCameraPlugin;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct CameraHolder;

// #[derive(Component, Default, Debug, Clone, Copy)]
// pub struct FpsCam {
//     pub yaw: f32,
//     pub pitch: f32,
// }

/// Global configuration for the camera. modify the resource of this
/// type to change from the default configuration
#[derive(Resource, Clone, Copy, Debug)]
pub struct CameraConfig {
    pub sensitivity: f32,
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self { sensitivity: 0.2 } // old 0.001
    }
}

impl Plugin for MyCameraPlugin {
    fn build(&self, app: &mut App) {
        app //
            .init_resource::<CameraConfig>()
            .insert_resource(CameraDistance(4.))
            .add_systems(Startup, setup_camera)
            .add_systems(
                Update,
                ((
                    change_camera_distance_with_mousewheel,
                    // adjust_camera_distance,
                )
                    .chain(),)
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                Update,
                (
                    mouse_motion,
                    // cameraholder_follow_player
                ),
            )
            .add_systems(OnEnter(AppState::InGame), toggle_cursor_grab)
            .add_systems(OnExit(AppState::InGame), toggle_cursor_grab);
    }
}

fn setup_camera(mut commands: Commands) {
    // MainCamera
    commands
        .spawn((Transform::from_xyz(0., 6., 0.), CameraHolder))
        .with_children(|parent| {
            parent.spawn((
                Camera3d::default(),
                // Camera {
                //     ..default()
                // },
                Transform::default().looking_at(Vec3::ZERO, Vec3::Y),
                MainCamera,
            ));
        });
}

#[derive(Resource)]
struct CameraDistance(f32);

fn change_camera_distance_with_mousewheel(
    mut camera_distance: ResMut<CameraDistance>,
    mut evr_scroll: EventReader<MouseWheel>,
) {
    use bevy::input::mouse::MouseScrollUnit;
    for ev in evr_scroll.read() {
        match ev.unit {
            MouseScrollUnit::Line => {
                // println!(
                //     "Scroll (line units): vertical: {}, horizontal: {}",
                //     ev.y, ev.x
                // );
                camera_distance.0 = (camera_distance.0 + -ev.y * 0.25).clamp(1., 100.);
            }
            MouseScrollUnit::Pixel => {
                // println!(
                //     "Scroll (pixel units): vertical: {}, horizontal: {}",
                //     ev.y, ev.x
                // );
                info!("Not implemented.");
            }
        }
    }
}

// const ADJUST_OFFSET: f32 = 0.5;

// fn adjust_camera_distance(
//     cameraholder_q: Query<&GlobalTransform, (With<CameraHolder>, Without<MainCamera>)>,
//     mut camera_q: Query<&mut Transform, (With<MainCamera>, Without<CameraHolder>)>,
//     player_q: Query<Entity, With<Player>>,
//     rapier_context: Res<RapierContext>,
//     camera_distance: Res<CameraDistance>,
// ) {
//     let cameraholder_globaltransform = cameraholder_q.single();
//     let mut camera_transform = camera_q.single_mut();

//     let ray_pos = cameraholder_globaltransform.translation();
//     let ray_dir = cameraholder_globaltransform.back();

//     if let Some((_entity, toi)) = rapier_context.cast_ray(
//         ray_pos,
//         ray_dir.into(),
//         camera_distance.0 + ADJUST_OFFSET,
//         false,
//         QueryFilter::exclude_dynamic()
//             .exclude_sensors()
//             .exclude_rigid_body(player_q.single()),
//     ) {
//         camera_transform.translation.z =
//             toi - (ADJUST_OFFSET * toi / (camera_distance.0 + ADJUST_OFFSET));
//     } else {
//         camera_transform.translation.z = camera_distance.0;
//     }
// }

// fn cameraholder_follow_player(
//     player_q: Query<&Transform, (With<Player>, Without<MainCamera>)>,
//     mut cameraholder_q: Query<&mut Transform, (With<CameraHolder>, Without<Player>)>,
// ) {
//     match player_q.get_single() {
//         Ok(Transform {
//             translation: player_translation,
//             ..
//         }) => {
//             cameraholder_q.single_mut().translation = *player_translation + Vec3::new(0., 0.85, 0.);
//         }
//         Err(QuerySingleError::NoEntities(_)) => {
//             println!("Error: There is no player!");
//         }
//         Err(QuerySingleError::MultipleEntities(_)) => {
//             println!("Error: There is more than one player!");
//         }
//     }
// }

fn toggle_cursor_grab(mut q_primary_window: Query<&mut Window, With<PrimaryWindow>>) {
    let mut primary_window = q_primary_window.single_mut();

    if primary_window.cursor_options.visible {
        primary_window.cursor_options.grab_mode = CursorGrabMode::Locked;
        primary_window.cursor_options.visible = false;
    } else {
        primary_window.cursor_options.grab_mode = CursorGrabMode::None;
        primary_window.cursor_options.visible = true;
    };
}

// mose_motion on update without run_if to clear the events when the cursor is not locked, to prevent buffering input.
fn mouse_motion(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut cameraholder_q: Query<&mut Transform, With<CameraHolder>>,
    camera_config: Res<CameraConfig>,
    q_primary_window: Query<&Window, With<PrimaryWindow>>,
) {
    let primary_window = q_primary_window.single();

    if primary_window.cursor_options.grab_mode != CursorGrabMode::Locked {
        mouse_motion_events.clear();
        return;
    }

    let mut mouse_delta = Vec2::ZERO;

    for mouse_event in mouse_motion_events.read() {
        mouse_delta += mouse_event.delta;
    }

    if mouse_delta != Vec2::ZERO {
        // mouse_delta.length_squared() > 0.0
        let mut camera_transform = cameraholder_q.single_mut();
        // Using smallest of height or width ensures equal vertical and horizontal sensitivity
        // let window_scale = primary_window.height().min(primary_window.width());

        let (mut yaw, mut pitch, _) = camera_transform.rotation.to_euler(EulerRot::YXZ);

        const RADIANS_PER_DOT: f32 = 1.0 / 180.0; // / window_scale * PI

        yaw -= mouse_delta.x * RADIANS_PER_DOT * camera_config.sensitivity; // 1.0 / 180.0
        pitch -= mouse_delta.y * RADIANS_PER_DOT * camera_config.sensitivity; // / window_scale * PI

        pitch = pitch.clamp(-1.56, 1.56); // PI / 2.

        camera_transform.rotation =
            Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
    }
}

// old mouse_motion
// fn orientation(
//     mut input: EventReader<MouseMotion>,
//     mut query: Query<(&mut Transform, &mut FpsCam)>,
//     config: Res<CameraConfig>,
// ) {
//     let (ref mut transform, ref mut fps_cam) = query.single_mut();
//     let mut delta_yaw = 0.;
//     let mut delta_pitch = 0.;

//     for ev in input.read() {
//         delta_yaw += ev.delta.x;
//         delta_pitch += ev.delta.y;
//     }
//     fps_cam.yaw -= delta_yaw * config.sensitivity;
//     fps_cam.pitch = (fps_cam.pitch - delta_pitch * config.sensitivity)
//         .clamp(-std::f32::consts::PI / 2.0, std::f32::consts::PI / 2.0);

//     transform.rotation =
//         Quat::from_axis_angle(Vec3::Y, fps_cam.yaw) * Quat::from_axis_angle(Vec3::X, fps_cam.pitch);
// }
