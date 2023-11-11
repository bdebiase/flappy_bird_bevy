use std::f32::consts::PI;

use bevy::{
    prelude::*,
    sprite::{Anchor, MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_asset_loader::{asset_collection::AssetCollection, loading_state::LoadingStateAppExt};

use crate::{anchor::AnchoredSprite, game::GameState, tiling::{Tiling, Parallax}};

#[derive(AssetCollection, Resource)]
struct BackgroundAssets {
    #[asset(path = "sprites/background/mountains.png")]
    pub mountains: Handle<Image>,
    #[asset(path = "sprites/background/buildings.png")]
    pub buildings: Handle<Image>,
    #[asset(path = "sprites/background/clouds.png")]
    pub clouds: Handle<Image>,
}

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_collection_to_loading_state::<_, BackgroundAssets>(GameState::Loading)
            .add_systems(OnExit(GameState::Loading), setup);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    background_assets: Res<BackgroundAssets>,
) {
    let mesh = Mesh::from(shape::Quad::default());
    let mesh_handle: Mesh2dHandle = meshes.add(mesh.clone()).into();

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: mesh_handle,
            material: materials.add(ColorMaterial::from(background_assets.mountains.clone())),
            transform: Transform::from_translation(Vec3::Z * -25.0),
            ..default()
        },
        AnchoredSprite {
            position: Anchor::BottomCenter,
            pivot: Anchor::TopCenter,
        },
        Tiling::default(),
        Parallax {
            ratio: 1. / 4.,
        },
    ));

    let mesh_handle: Mesh2dHandle = meshes.add(mesh.clone()).into();
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: mesh_handle,
            material: materials.add(ColorMaterial::from(background_assets.buildings.clone())),
            transform: Transform::from_translation(Vec3::Z * -50.0),
            ..default()
        },
        AnchoredSprite {
            position: Anchor::BottomCenter,
            pivot: Anchor::TopCenter,
        },
        Tiling::default(),
        Parallax {
            ratio: 1. / 8.,
        },
    ));

    let mesh_handle: Mesh2dHandle = meshes.add(mesh.clone()).into();
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: mesh_handle,
            material: materials.add(ColorMaterial::from(background_assets.clouds.clone())),
            transform: Transform::from_translation(Vec3::Z * -75.0),
            ..default()
        },
        AnchoredSprite {
            position: Anchor::BottomCenter,
            pivot: Anchor::TopCenter,
        },
        Tiling::default(),
        Parallax {
            ratio: 1. / 16.,
        },
    ));

    let mesh_handle: Mesh2dHandle = meshes.add(mesh).into();
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: mesh_handle,
            material: materials.add(ColorMaterial::from(background_assets.clouds.clone())),
            transform: Transform::from_translation(Vec3::Z * 10.0)
                .with_rotation(Quat::from_rotation_z(PI)),
            ..default()
        },
        AnchoredSprite {
            position: Anchor::TopCenter,
            pivot: Anchor::TopCenter,
        },
        Tiling::default(),
        Parallax {
            ratio: -1. / 6.,
        },
    ));
}
