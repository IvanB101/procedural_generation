use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
    sprite::MaterialMesh2dBundle,
    utils::info,
    window::PrimaryWindow,
};
use bevy_inspector_egui::InspectorOptions;
use bevy_inspector_egui::{prelude::*, quick::ResourceInspectorPlugin};
use procedural_generation::{
    common::CommonPlugin,
    utils::noise::{perlin::Perlin, Noise},
};

#[derive(Component)]
struct NoiseImage;

#[derive(Reflect, Resource, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
struct NoiseConfiguration {
    layers: Vec<(f32, f32)>,
    wrap: usize,
    // #[inspector(min = 0.0, max = 1.0)]
}

impl Default for NoiseConfiguration {
    fn default() -> Self {
        NoiseConfiguration {
            layers: vec![(0.5, 1.), (0.25, 2.), (0.125, 4.), (0.075, 8.)],
            wrap: 256,
        }
    }
}

#[derive(Reflect, Resource, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
struct Configuration {
    #[inspector(min = 0.0, max = 1.0)]
    compress_factor: f32,
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            compress_factor: 0.03,
        }
    }
}

#[derive(Resource)]
struct Global {
    noise: Perlin,
    width: u32,
    height: u32,
}

impl Default for Global {
    fn default() -> Self {
        let NoiseConfiguration { layers, wrap, .. } = NoiseConfiguration::default();

        Global {
            noise: Perlin::new(&layers, wrap),
            width: 1920,
            height: 1080,
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Procedural Generation".into(),
                visible: false, // false to make visible on Startup. Prevents long white window on start
                present_mode: bevy::window::PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        .init_resource::<Configuration>()
        .init_resource::<NoiseConfiguration>()
        .init_resource::<Global>()
        .register_type::<Configuration>()
        .register_type::<NoiseConfiguration>()
        .add_plugins(ResourceInspectorPlugin::<Configuration>::default())
        .add_plugins(ResourceInspectorPlugin::<NoiseConfiguration>::default())
        .add_plugins(CommonPlugin)
        .add_plugins(bevy_framepace::FramepacePlugin)
        .add_systems(Startup, (setup, make_visible))
        .add_systems(Update, update)
        // .add_systems(Update, resize)
        .run();
}

// Make visible on Startup. Prevents long white window on start
fn make_visible(mut window: Query<&mut Window, With<PrimaryWindow>>) {
    window.single_mut().visible = true;
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut images: ResMut<Assets<Image>>,
    config: Res<Configuration>,
    global: Res<Global>,
) {
    let noise = &global.noise;

    let factor = config.compress_factor;
    let mut colors = Vec::new();

    for y in 0..global.height {
        for x in 0..global.width {
            let value = (noise.get(&[x as f32 * factor, y as f32 * factor]) * 256.) as u8;

            colors.push(value);
            colors.push(value);
            colors.push(value);
            colors.push(255);
        }
    }

    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::default()).into(),
            transform: Transform::default().with_scale(Vec3 {
                x: global.width as f32,
                y: global.height as f32,
                z: 1.,
            }),
            material: materials.add(ColorMaterial {
                color: Color::srgb(1., 1., 1.),
                texture: Some(images.add(Image::new_fill(
                    Extent3d {
                        width: global.width,
                        height: global.height,
                        depth_or_array_layers: 1,
                    },
                    TextureDimension::D2,
                    &colors,
                    TextureFormat::Rgba8Unorm,
                    RenderAssetUsages::RENDER_WORLD,
                ))),
            }),
            ..default()
        },
        NoiseImage,
    ));
}

fn update(
    mut mesh_query: Query<&mut Handle<ColorMaterial>, With<NoiseImage>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    config: Res<Configuration>,
    noise_config: Res<NoiseConfiguration>,
    mut global: ResMut<Global>,
) {
    if !config.is_changed() && !noise_config.is_changed() {
        return;
    }

    if noise_config.is_changed() {
        global.noise.set_layers(&noise_config.layers);
        global.noise.set_wrap(noise_config.wrap);
    }

    let noise = &global.noise;

    let factor = config.compress_factor;
    let mut colors = Vec::new();

    for y in 0..global.height {
        for x in 0..global.width {
            let value = (noise.get(&[x as f32 * factor, y as f32 * factor]) * 256.) as u8;

            colors.push(value);
            colors.push(value);
            colors.push(value);
            colors.push(255);
        }
    }

    let material = mesh_query.get_single_mut();

    if material.is_err() {
        info(material);
        return;
    }

    *material.unwrap() = materials.add(ColorMaterial {
        color: Color::srgb(1., 1., 1.),
        texture: Some(images.add(Image::new_fill(
            Extent3d {
                width: global.width,
                height: global.height,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            &colors,
            TextureFormat::Rgba8Unorm,
            RenderAssetUsages::RENDER_WORLD,
        ))),
    });
}
