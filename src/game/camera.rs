use bevy::{
    color::palettes::css::WHITE,
    input::{keyboard::KeyboardInput, mouse::MouseMotion},
    prelude::*,
    window::CursorGrabMode,
};

use crate::AppState;

#[derive(Component)]
pub struct FpsCamPlugin;

#[derive(Component)]
pub struct GameCamera;

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct FpsCam {
    pub yaw: f32,
    pub pitch: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct KeyBindings {
    pub forward: Option<KeyCode>,
    pub back: Option<KeyCode>,
    pub left: Option<KeyCode>,
    pub right: Option<KeyCode>,
    pub up: Option<KeyCode>,
    pub down: Option<KeyCode>,
    pub unlock: Option<KeyCode>,
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            forward: Some(KeyCode::KeyW),
            back: Some(KeyCode::KeyS),
            left: Some(KeyCode::KeyA),
            right: Some(KeyCode::KeyD),
            up: Some(KeyCode::Space),
            down: Some(KeyCode::ControlLeft),
            unlock: Some(KeyCode::Escape),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            movespeed: 1.0,
            sensitivity: 0.001,
            key_bindings: Default::default(),
        }
    }
}

/// Global configuration for the camera. modify the resource of this
/// type to change from the default configuration
#[derive(Resource, Clone, Copy, Debug)]
pub struct Config {
    pub movespeed: f32,
    pub sensitivity: f32,
    pub key_bindings: KeyBindings,
}

// fn lock_cursor(mut windows: ResMut<Window>, mut mouse_events: EventReader<MouseButtonInput>) {
//     let window = windows.get_primary_mut().unwrap();
//     for ev in mouse_events.iter() {
//         if ev.state == ElementState::Pressed {
//             set_cursor_lock(window, true);
//         }
//     }
// }

impl Plugin for FpsCamPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Config>()
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (movement, orientation).run_if(in_state(AppState::InGame)),
            )
            .add_systems(OnEnter(AppState::InGame), toggle_cursor_lock)
            .add_systems(OnExit(AppState::InGame), toggle_cursor_lock);
    }
}

fn toggle_cursor_lock(mut query: Query<&mut Window>) {
    let mut window = query.single_mut();
    let state = window.cursor.visible;

    window.cursor.visible = !state;
    window.cursor.grab_mode = if state {
        CursorGrabMode::Locked
    } else {
        CursorGrabMode::None
    };
}

fn setup(mut commands: Commands) {
    let position = Vec3::new(2., 2., 2.);
    let target = Vec3::ZERO;

    let camera_and_light_transform =
        Transform::from_translation(position).looking_at(target, Vec3::Y);
    let Vec3 { x, y, z } = position - target;
    let yaw = Vec3::new(x, 0., z).angle_between(Vec3::X);
    let pitch = -Vec3::new(x, y, 0.).angle_between(Vec3::Y);

    commands.spawn((
        GameCamera,
        FpsCam { yaw, pitch },
        Camera3dBundle {
            transform: camera_and_light_transform,
            ..default()
        },
    ));

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 100000.0,
            ..default()
        },
        transform: camera_and_light_transform,
        ..default()
    });

    commands.insert_resource(AmbientLight {
        color: WHITE.into(),
        brightness: 1.,
        ..default()
    })
}

fn movement(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<FpsCam>>,
    config: Res<Config>,
    time: Res<Time>,
) {
    let mut transform = query.single_mut();
    let Vec3 { x, z, .. } = transform.forward().into();
    let forward = Vec3::new(x, 0., z).normalize();
    let right = Vec3::new(-z, 0., x).normalize();
    let mut displacement = Vec3::new(0., 0., 0.);

    for key in keys.get_pressed() {
        match *key {
            x if Some(x) == config.key_bindings.forward => displacement += forward,
            x if Some(x) == config.key_bindings.back => displacement -= forward,
            x if Some(x) == config.key_bindings.left => displacement -= right,
            x if Some(x) == config.key_bindings.right => displacement += right,
            x if Some(x) == config.key_bindings.up => displacement += Vec3::Y,
            x if Some(x) == config.key_bindings.down => displacement -= Vec3::Y,
            _ => (),
        }
    }

    transform.translation +=
        displacement.normalize_or_zero() * config.movespeed * time.delta_seconds();
}

fn orientation(
    mut input: EventReader<MouseMotion>,
    mut query: Query<(&mut Transform, &mut FpsCam)>,
    config: Res<Config>,
) {
    let (ref mut transform, ref mut fps_cam) = query.single_mut();
    let mut delta_yaw = 0.;
    let mut delta_pitch = 0.;

    for ev in input.read() {
        delta_yaw += ev.delta.x;
        delta_pitch += ev.delta.y;
    }
    fps_cam.yaw -= delta_yaw * config.sensitivity;
    fps_cam.pitch = (fps_cam.pitch - delta_pitch * config.sensitivity)
        .clamp(-std::f32::consts::PI / 2.0, std::f32::consts::PI / 2.0);

    transform.rotation =
        Quat::from_axis_angle(Vec3::Y, fps_cam.yaw) * Quat::from_axis_angle(Vec3::X, fps_cam.pitch);
}
