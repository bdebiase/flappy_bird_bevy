use std::f32::consts::PI;

use bevy::{
    prelude::*,
    render::texture::{
        ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor,
    },
    sprite::{Anchor, MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::{anchor::AnchoredSprite, tiling::Tiling, GameState};

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::Loading), setup);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let mesh = Mesh::from(shape::Quad::default());

    let sampler_desc = ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        ..Default::default()
    };
    let settings = move |s: &mut ImageLoaderSettings| {
        s.sampler = ImageSampler::Descriptor(sampler_desc.clone());
    };

    let texture_handle =
        asset_server.load_with_settings("sprites/background/mountains.png", settings.clone());
    let mesh_handle: Mesh2dHandle = meshes.add(mesh.clone()).into();
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: mesh_handle,
            material: materials.add(ColorMaterial::from(texture_handle)),
            transform: Transform::from_translation(Vec3::Z * -25.0),
            ..default()
        },
        AnchoredSprite {
            position: Anchor::BottomCenter,
            pivot: Anchor::TopCenter,
        },
        Tiling {
            parallax_ratio: 1. / 4.,
        },
    ));

    let texture_handle =
        asset_server.load_with_settings("sprites/background/buildings.png", settings.clone());
    let mesh_handle: Mesh2dHandle = meshes.add(mesh.clone()).into();
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: mesh_handle,
            material: materials.add(ColorMaterial::from(texture_handle)),
            transform: Transform::from_translation(Vec3::Z * -50.0),
            ..default()
        },
        AnchoredSprite {
            position: Anchor::BottomCenter,
            pivot: Anchor::TopCenter,
        },
        Tiling {
            parallax_ratio: 1. / 8.,
        },
    ));

    let texture_handle =
        asset_server.load_with_settings("sprites/background/clouds.png", settings.clone());
    let mesh_handle: Mesh2dHandle = meshes.add(mesh.clone()).into();
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: mesh_handle,
            material: materials.add(ColorMaterial::from(texture_handle)),
            transform: Transform::from_translation(Vec3::Z * -75.0),
            ..default()
        },
        AnchoredSprite {
            position: Anchor::BottomCenter,
            pivot: Anchor::TopCenter,
        },
        Tiling {
            parallax_ratio: 1. / 16.,
        },
    ));

    let texture_handle = asset_server.load_with_settings("sprites/background/clouds.png", settings);
    let mesh_handle: Mesh2dHandle = meshes.add(mesh).into();
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: mesh_handle,
            material: materials.add(ColorMaterial::from(texture_handle)),
            transform: Transform::from_translation(Vec3::Z * 10.0)
                .with_rotation(Quat::from_rotation_z(PI)),
            ..default()
        },
        AnchoredSprite {
            position: Anchor::TopCenter,
            pivot: Anchor::TopCenter,
        },
        Tiling {
            parallax_ratio: -1. / 6.,
        },
    ));
}
