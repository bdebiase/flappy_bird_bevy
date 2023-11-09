use bevy::{prelude::*, sprite::{Mesh2dHandle, MaterialMesh2dBundle, Anchor}, render::texture::{ImageSamplerDescriptor, ImageAddressMode, ImageLoaderSettings, ImageSampler}};

use crate::{GameState, tiling::Tiling, anchor::AnchoredSprite};

#[derive(Component)]
pub struct Ground;

pub struct GroundPlugin;

impl Plugin for GroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::Loading), setup);
            // .add_systems(Update, reposition_ground);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
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

    let texture_handle = asset_server.load_with_settings("sprites/ground.png", settings);
    commands.spawn((MaterialMesh2dBundle {
        mesh: mesh_handle,
        material: materials.add(ColorMaterial::from(texture_handle)),
        transform: Transform::from_translation(Vec3::Z * 0.0),
        ..default()
    }, AnchoredSprite { position: Anchor::BottomCenter, pivot: Anchor::BottomCenter }, Tiling::default()));
}