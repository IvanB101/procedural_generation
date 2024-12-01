use bevy::{app::AppExit, prelude::*};

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app //
            .insert_resource(ScreenMode::BorderlessFullscreen)
            .add_systems(
                Update,
                (screen_mode_with_resource, set_screen_mode_with_keys),
            );

        #[cfg(debug_assertions)]
        {
            // app.add_systems(Update, close_on_esc);

            // app.add_plugins((
            //     bevy::diagnostic::LogDiagnosticsPlugin::default(),
            //     // bevy::diagnostic::FrameTimeDiagnosticsPlugin, // remove because of hud
            //     bevy::diagnostic::EntityCountDiagnosticsPlugin,
            // ));
        }
    }
}

// Config
// WindowMode
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub enum ScreenMode {
    Windowed,
    BorderlessFullscreen,
}

#[allow(dead_code)]
fn close_on_esc(
    mut app_exit_events: EventWriter<AppExit>,
    // mut commands: Commands,
    focused_windows: Query<(Entity, &Window)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (_window, focus) in focused_windows.iter() {
        if !focus.focused {
            continue;
        }

        if input.just_pressed(KeyCode::Escape) {
            // commands.entity(window).despawn();
            app_exit_events.send(AppExit::Success);
        }
    }
}

fn set_screen_mode_with_keys(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut screen_mode: ResMut<ScreenMode>,
) {
    if (keyboard_input.pressed(KeyCode::AltLeft) && keyboard_input.just_pressed(KeyCode::Enter))
        || keyboard_input.just_pressed(KeyCode::KeyF)
    {
        match *screen_mode {
            ScreenMode::Windowed => {
                *screen_mode = ScreenMode::BorderlessFullscreen;
            }
            ScreenMode::BorderlessFullscreen => {
                *screen_mode = ScreenMode::Windowed;
            }
        }
    }
}

fn screen_mode_with_resource(screen_mode: Res<ScreenMode>, mut windows: Query<&mut Window>) {
    if screen_mode.is_changed() {
        let mut window = windows.single_mut();
        match *screen_mode {
            ScreenMode::BorderlessFullscreen => {
                window.mode = bevy::window::WindowMode::BorderlessFullscreen(MonitorSelection::Current)
            }
            ScreenMode::Windowed => window.mode = bevy::window::WindowMode::Windowed,
        }
    }
}
