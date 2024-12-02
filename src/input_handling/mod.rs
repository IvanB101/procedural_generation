use bevy::prelude::*;

use crate::AppState;

pub struct InputHandlingPlugin;

#[derive(Resource)]
pub struct KeyBindings {
    pub forward: Option<KeyCode>,
    pub back: Option<KeyCode>,
    pub left: Option<KeyCode>,
    pub right: Option<KeyCode>,
    pub jump: Option<KeyCode>,
    pub down: Option<KeyCode>,
    pub sprint: Option<KeyCode>,
    pub pause: Option<KeyCode>,
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            forward: Some(KeyCode::KeyW),
            back: Some(KeyCode::KeyS),
            left: Some(KeyCode::KeyA),
            right: Some(KeyCode::KeyD),
            jump: Some(KeyCode::Space),
            down: Some(KeyCode::ControlLeft),
            sprint: Some(KeyCode::ShiftLeft),
            pause: Some(KeyCode::Escape),
        }
    }
}

impl Plugin for InputHandlingPlugin {
    fn build(&self, app: &mut App) {
        app //
            .insert_resource(KeyBindings::default())
            .add_systems(Update, toggle_menu);
    }
}

fn toggle_menu(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
    keybindings: Res<KeyBindings>,
    mut virtual_time: ResMut<Time<Virtual>>,
) {
    if let Some(pause_key) = keybindings.pause {
        if keyboard_input.just_pressed(pause_key) {
            match state.get() {
                AppState::InGame => {
                    virtual_time.pause();
                    next_state.set(AppState::MainMenu);
                }
                AppState::MainMenu => {
                    virtual_time.unpause();
                    next_state.set(AppState::InGame);
                }
            }
        }
    }
}
