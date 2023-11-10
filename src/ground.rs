use bevy::{
    prelude::*,
    render::texture::{
        ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor,
    },
    sprite::{Anchor, MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_asset_loader::{asset_collection::AssetCollection, loading_state::LoadingStateAppExt};

use crate::{anchor::AnchoredSprite, game::GameState, tiling::Tiling};

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
    ground_assets: Res<GroundAssets>,
) {
    let mesh = Mesh::from(shape::Quad::default());
    let mesh_handle: Mesh2dHandle = meshes.add(mesh).into();

    let sampler_desc = ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        ..Default::default()
    };
    let settings = move |s: &mut ImageLoaderSettings| {
        s.sampler = ImageSampler::Descriptor(sampler_desc.clone());
    };

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
        },
        Tiling::default(),
    ));
}
