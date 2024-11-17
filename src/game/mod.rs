use bevy::prelude::*;

use crate::AppState;

use self::camera::{FpsCamPlugin, GameCamera};
use self::map::Map;

mod camera;
mod map;

#[derive(Component)]
pub struct DebugText;

#[derive(Component)]
pub struct Game;

impl Plugin for Game {
    fn build(&self, app: &mut App) {
        app.add_plugins((FpsCamPlugin, Map))
            .add_systems(Startup, setup)
            .add_systems(Update, (update_text).run_if(in_state(AppState::InGame)));
    }
}

fn setup(mut commands: Commands) {
    // Import the custom texture.
    // let custom_texture_handle: Handle<Image> = asset_server.load("textures/array_texture.png");
    // Create and save a handle to the mesh.
    commands.spawn((
        DebugText,
        TextBundle::from_section(
            format!("Position: \n Orientation: \n"),
            TextStyle {
                font_size: 20.,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        }),
    ));
}

fn update_text(
    mut text_query: Query<&mut Text, With<DebugText>>,
    query: Query<&Transform, With<GameCamera>>,
) {
    let transform = query.single();
    let mut text = text_query.single_mut();

    text.sections[0].value = format!(
        "Position: {:?}\nQuat:{:?} \n",
        transform.translation, transform.rotation,
    );
}
