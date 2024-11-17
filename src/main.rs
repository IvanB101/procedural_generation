use bevy::prelude::*;
#[allow(unused_imports)]
#[cfg(debug_assertions)]
use bevy_dylib;
use game::Game;

mod game;
mod noise;
mod ui;

#[derive(Default, States, Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    MainMenu,
    #[default]
    InGame,
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Game))
        .init_state::<AppState>()
        .add_systems(Update, toggle_menu)
        .run();
}

fn toggle_menu(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
    state: ResMut<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(match state.get() {
            AppState::InGame => AppState::MainMenu,
            AppState::MainMenu => AppState::InGame,
        })
    }
}
