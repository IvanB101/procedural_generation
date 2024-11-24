use bevy::{
    input::common_conditions::input_toggle_active, prelude::*, sprite::MaterialMesh2dBundle,
    window::PrimaryWindow,
};
use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use procedural_generation::common::CommonPlugin;

#[derive(Component)]
struct NoiseImage;

pub struct ImageWrapper {
    pub image: Image,
}

pub struct TransformWrapper {
    pub transform: Transform,
}

pub fn noise_playground<'a, C, G>()
where
    C: Resource + Default + Reflect + Clone,
    G: Resource + Default + Clone,
    (C, G): Into<ImageWrapper>,
    (C, G): Into<TransformWrapper>,
{
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Generation".into(),
                visible: false, // false to make visible on Startup. Prevents long white window on start
                present_mode: bevy::window::PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        .init_resource::<G>()
        .init_resource::<C>()
        .add_plugins(
            ResourceInspectorPlugin::<C>::default()
                .run_if(input_toggle_active(true, KeyCode::Escape)),
        )
        .add_plugins(CommonPlugin)
        .add_plugins(bevy_framepace::FramepacePlugin)
        .add_systems(Startup, (setup::<C, G>, make_visible))
        .add_systems(Update, update::<C, G>)
        // .add_systems(Update, resize)
        .run();
}

// Make visible on Startup. Prevents long white window on start
fn make_visible(mut window: Query<&mut Window, With<PrimaryWindow>>) {
    window.single_mut().visible = true;
}

fn setup<'a, C, G>(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut images: ResMut<Assets<Image>>,
    config: Res<C>,
    global: Res<G>,
) where
    C: Resource + Default + Reflect + Clone,
    G: Resource + Default + Clone,
    (C, G): Into<ImageWrapper>,
    (C, G): Into<TransformWrapper>,
{
    let transform_wrapper: TransformWrapper = (config.clone(), global.clone()).into();
    let wrapper: ImageWrapper = (config.clone(), global.clone()).into();

    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::default()).into(),
            transform: transform_wrapper.transform,
            material: materials.add(ColorMaterial {
                color: Color::srgb(1., 1., 1.),
                texture: Some(images.add(wrapper.image)),
            }),
            ..default()
        },
        NoiseImage,
    ));
}

fn update<'a, C, G>(
    mut mesh_query: Query<&mut Handle<ColorMaterial>, With<NoiseImage>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    config: Res<C>,
    global: Res<G>,
) where
    C: Resource + Default + Reflect + Clone,
    G: Resource + Default + Clone,
    (C, G): Into<ImageWrapper>,
{
    if !config.is_changed() {
        return;
    }

    let material = mesh_query.get_single_mut();

    if material.is_err() {
        return;
    }

    let wrapper: ImageWrapper = (config.clone(), global.clone()).into();

    *material.unwrap() = materials.add(ColorMaterial {
        color: Color::srgb(1., 1., 1.),
        texture: Some(images.add(wrapper.image)),
    });
}
