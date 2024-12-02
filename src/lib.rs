use bevy::prelude::States;

pub mod camera;
pub mod common;
pub mod input_handling;
pub mod player;
pub mod shaders;
pub mod terrain;
pub mod ui;
pub mod utils;

#[derive(Default, States, Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    #[default]
    InGame,
}
