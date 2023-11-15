use bevy::{
    prelude::*,
    sprite::{Anchor, MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_asset_loader::{asset_collection::AssetCollection, loading_state::LoadingStateAppExt};
use bevy_xpbd_2d::components::{RigidBody, Collider};

use crate::{
    anchor::AnchoredSprite,
    game::{GameState, GameBoundaries},
    tiling::{Parallax, Tiling},
};

#[derive(Component)]
pub struct Ground;

#[derive(AssetCollection, Resource)]
struct GroundAssets {
    #[asset(path = "sprites/ground.png")]
    image: Handle<Image>,
}

pub struct GroundPlugin;

impl Plugin for GroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_collection_to_loading_state::<_, GroundAssets>(GameState::Loading)
            .add_systems(OnExit(GameState::Loading), setup);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    game_boundaries: Res<GameBoundaries>,
    ground_assets: Res<GroundAssets>,
) {
    let mesh = Mesh::from(shape::Quad::default());
    let mesh_handle: Mesh2dHandle = meshes.add(mesh).into();

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: mesh_handle,
            material: materials.add(ColorMaterial::from(ground_assets.image.clone())),
            transform: Transform::from_translation(Vec3::Z * 0.0),
            ..default()
        },
        AnchoredSprite {
            position: Anchor::BottomCenter,
            pivot: Anchor::BottomCenter,
            stretch: true,
        },
        Tiling {
            tile_x: true,
            ..default()
        },
        Parallax::default(),
        RigidBody::Static,
        Collider::segment(Vec2::new(-100.0, 0.0), Vec2::new(100.0, 0.0)),
    ));
}
