use bevy::{
    color::palettes::css::LIME,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
    window::PrimaryWindow,
};

pub struct HUDPlugin;

#[derive(Component)]
struct FpsText;

#[derive(Component)]
struct Crosshair;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app //
            .add_plugins(FrameTimeDiagnosticsPlugin)
            .add_systems(Startup, setup_hud)
            .add_systems(Update, (crosshair_visibility, fps_text_update_system));
    }
}

fn setup_hud(mut commands: Commands) {
    // root node
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..Default::default()
        })
        .with_children(|parent| {
            // Crosshair
            parent
                .spawn((
                    Node {
                        width: Val::Px(8.),
                        height: Val::Px(8.),
                        position_type: PositionType::Absolute,
                        border: UiRect::all(Val::Px(1.)),
                        ..Default::default()
                    },
                    BackgroundColor(Color::WHITE),
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                ))
                .insert(Crosshair);
        });

    commands
        .spawn((Text::default(), BackgroundColor(Color::BLACK)))
        .with_child((
            TextSpan::new(""),
            TextColor(LIME.into()),
            TextFont::from_font_size(20.),
            FpsText,
        ));
}

fn crosshair_visibility(
    q_primary_window: Query<&Window, With<PrimaryWindow>>,
    mut crosshair_q: Query<&mut Visibility, With<Crosshair>>,
) {
    let primary_window_result = q_primary_window.get_single();

    let primary_window = if primary_window_result.is_err() {
        return;
    } else {
        primary_window_result.unwrap()
    };

    let mut crosshair_visibility = crosshair_q.single_mut();

    match primary_window.cursor_options.grab_mode {
        bevy::window::CursorGrabMode::Locked => {
            *crosshair_visibility = Visibility::Visible;
        }
        _ => {
            *crosshair_visibility = Visibility::Hidden;
        }
    }
}

fn fps_text_update_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut TextSpan, With<FpsText>>,
) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                // Update the fps text
                text.0 = format!("{value:.0}");
            }
        }
    }
}

// fn speed_text_update_system(
//     mut query: Query<&mut Text, With<SpeedText>>,
//     game_speed: Res<GameSpeed>,
// ) {
//     for mut text in &mut query {
//         text.sections[1].value = format!("x{:.2}", game_speed.speed);
//     }
// }
