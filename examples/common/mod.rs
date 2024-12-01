use bevy::{
    input::common_conditions::input_toggle_active,
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
    window::PrimaryWindow,
};
use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use procedural_generation::common::CommonPlugin;

pub struct VecWrapper<T> {
    pub vec: Vec<T>,
}

pub const IMAGE_DIMENSIONS: (u32, u32) = (1920, 1080);

#[derive(Component)]
struct NoiseImage;

pub fn noise_playground<C>()
where
    C: Resource + Default + Reflect + Clone + Into<VecWrapper<u8>>,
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
        .init_resource::<C>()
        .add_plugins(
            ResourceInspectorPlugin::<C>::default()
                .run_if(input_toggle_active(true, KeyCode::Escape)),
        )
        .add_plugins(CommonPlugin)
        // .add_plugins(bevy_framepace::FramepacePlugin)
        .add_systems(Startup, (setup, make_visible))
        .add_systems(Update, update::<C>)
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
) {
    let transform = Transform::default().with_scale(Vec3 {
        x: IMAGE_DIMENSIONS.0 as f32,
        y: IMAGE_DIMENSIONS.1 as f32,
        z: 1.,
    });

    commands.spawn(Camera2d::default());

    commands.spawn((
        transform,
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(ColorMaterial {
            color: Color::srgb(1., 1., 1.),
            texture: Some(images.add(Image::new_fill(
                Extent3d {
                    width: IMAGE_DIMENSIONS.0,
                    height: IMAGE_DIMENSIONS.1,
                    depth_or_array_layers: 1,
                },
                TextureDimension::D2,
                &[0, 0, 0, 0],
                TextureFormat::Rgba8Unorm,
                RenderAssetUsages::RENDER_WORLD,
            ))),
            // alpha_mode: bevy::sprite::AlphaMode2d::Blend,
            ..Default::default()
        })),
        NoiseImage,
    ));
}

fn update<C>(
    mut mesh_query: Query<&mut MeshMaterial2d<ColorMaterial>, With<NoiseImage>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    config: Res<C>,
) where
    C: Resource + Default + Reflect + Clone + Into<VecWrapper<u8>>,
{
    if !config.is_changed() {
        return;
    }

    let material = mesh_query.get_single_mut();

    if material.is_err() {
        return;
    }

    let colors = config.clone().into();

    let image = Image::new_fill(
        Extent3d {
            width: IMAGE_DIMENSIONS.0,
            height: IMAGE_DIMENSIONS.1,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &colors.vec,
        TextureFormat::Rgba8Unorm,
        RenderAssetUsages::RENDER_WORLD,
    );

    material.unwrap().0 = materials.add(ColorMaterial {
        color: Color::srgb(1., 1., 1.),
        texture: Some(images.add(image)),
        ..Default::default()
    });
}
