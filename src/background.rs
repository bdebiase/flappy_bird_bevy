use std::f32::consts::PI;

use bevy::{
    prelude::*,
    sprite::{Anchor, MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_asset_loader::{asset_collection::AssetCollection, loading_state::LoadingStateAppExt};

use crate::{
    anchor::AnchoredSprite,
    game::{GameAssets, GameState},
    tiling::{Parallax, Tiling},
};

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
    game_assets: Res<GameAssets>,
) {
    let mesh = Mesh::from(shape::Quad::default());
    let mesh_handle: Mesh2dHandle = meshes.add(mesh.clone()).into();

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: mesh_handle,
            material: materials.add(ColorMaterial::from(game_assets.mountains_image.clone())),
            transform: Transform::from_translation(Vec3::Z * -25.0),
            ..default()
        },
        AnchoredSprite {
            position: Anchor::BottomCenter,
            pivot: Anchor::TopCenter,
            ..default()
        },
        Tiling {
            tile_x: true,
            ..default()
        },
        Parallax { ratio: 1. / 4. },
    ));

    let mesh_handle: Mesh2dHandle = meshes.add(mesh.clone()).into();
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: mesh_handle,
            material: materials.add(ColorMaterial::from(game_assets.buildings_image.clone())),
            transform: Transform::from_translation(Vec3::Z * -50.0),
            ..default()
        },
        AnchoredSprite {
            position: Anchor::BottomCenter,
            pivot: Anchor::TopCenter,
            ..default()
        },
        Tiling {
            tile_x: true,
            ..default()
        },
        Parallax { ratio: 1. / 8. },
    ));

    let mesh_handle: Mesh2dHandle = meshes.add(mesh.clone()).into();
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: mesh_handle,
            material: materials.add(ColorMaterial::from(game_assets.clouds_image.clone())),
            transform: Transform::from_translation(Vec3::Z * -75.0),
            ..default()
        },
        AnchoredSprite {
            position: Anchor::BottomCenter,
            pivot: Anchor::TopCenter,
            ..default()
        },
        Tiling {
            tile_x: true,
            ..default()
        },
        Parallax { ratio: 1. / 16. },
    ));

    let mesh_handle: Mesh2dHandle = meshes.add(mesh).into();
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: mesh_handle,
            material: materials.add(ColorMaterial::from(game_assets.clouds_image.clone())),
            transform: Transform::from_translation(Vec3::Z * 10.0)
                .with_rotation(Quat::from_rotation_z(PI)),
            ..default()
        },
        AnchoredSprite {
            position: Anchor::TopCenter,
            pivot: Anchor::TopCenter,
            ..default()
        },
        Tiling {
            tile_x: true,
            ..default()
        },
        Parallax { ratio: -1. / 6. },
    ));
}
