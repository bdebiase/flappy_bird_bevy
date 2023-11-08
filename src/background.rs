use std::f32::consts::PI;

use bevy::{prelude::*, asset::LoadedFolder, window::WindowResized, render::texture::{ImageSamplerDescriptor, ImageAddressMode, ImageSampler, ImageLoaderSettings}, sprite::{Mesh2dHandle, MaterialMesh2dBundle}};

use crate::{GameState, ground::Ground, tiling::Tiling, GameExtents};

#[derive(Component, Default)]
pub struct Background {
    pub anchor: usize,
    pub pivot: Vec2,
}

impl Background {
    pub const ANCHOR_TOP: usize = 0;
    pub const ANCHOR_BOTTOM: usize = 1;
    pub const ANCHOR_LEFT: usize = 2;
    pub const ANCHOR_RIGHT: usize = 3;
}

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::Loading), setup)
            .add_systems(PostUpdate, reposition_background);
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

    let texture_handle = asset_server.load_with_settings("sprites/background/mountains.png", settings.clone());
    let mesh_handle: Mesh2dHandle = meshes.add(mesh.clone()).into();
    commands.spawn((MaterialMesh2dBundle {
        mesh: mesh_handle,
        material: materials.add(ColorMaterial::from(texture_handle)),
        transform: Transform::from_translation(Vec3::Z * -25.0),
        ..default()
    }, Background { anchor: Background::ANCHOR_BOTTOM, pivot: Vec2::new(0.0, 1.0) }, Tiling { parallax_ratio: 1./4. }));

    let texture_handle = asset_server.load_with_settings("sprites/background/buildings.png", settings.clone());
    let mesh_handle: Mesh2dHandle = meshes.add(mesh.clone()).into();
    commands.spawn((MaterialMesh2dBundle {
        mesh: mesh_handle,
        material: materials.add(ColorMaterial::from(texture_handle)),
        transform: Transform::from_translation(Vec3::Z * -50.0),
        ..default()
    }, Background { anchor: Background::ANCHOR_BOTTOM, pivot: Vec2::new(0.0, 1.0) }, Tiling { parallax_ratio: 1./16. }));

    let texture_handle = asset_server.load_with_settings("sprites/background/clouds.png", settings.clone());
    let mesh_handle: Mesh2dHandle = meshes.add(mesh.clone()).into();
    commands.spawn((MaterialMesh2dBundle {
        mesh: mesh_handle,
        material: materials.add(ColorMaterial::from(texture_handle)),
        transform: Transform::from_translation(Vec3::Z * -75.0),
        ..default()
    }, Background { anchor: Background::ANCHOR_BOTTOM, pivot: Vec2::new(0.0, 1.0) }, Tiling { parallax_ratio: 1./32. }));

    let texture_handle = asset_server.load_with_settings("sprites/background/clouds.png", settings);
    let mesh_handle: Mesh2dHandle = meshes.add(mesh).into();
    commands.spawn((MaterialMesh2dBundle {
        mesh: mesh_handle,
        material: materials.add(ColorMaterial::from(texture_handle)),
        transform: Transform::from_translation(Vec3::Z * 10.0).with_rotation(Quat::from_rotation_z(PI)),
        ..default()
    }, Background { anchor: Background::ANCHOR_TOP, pivot: Vec2::new(0.0, -2.0) }, Tiling { parallax_ratio: -1./8. }));
}

fn reposition_background(
    mut query: Query<(&mut Transform, &Handle<ColorMaterial>, &Background)>,
    game_extents: Res<GameExtents>,
    materials: Res<Assets<ColorMaterial>>,
    images: Res<Assets<Image>>,
) {
    query.for_each_mut(|(mut transform, material_handle, background)| {
        let material = materials.get(material_handle).unwrap();
        let image_handle = material.texture.clone().unwrap();
        if let Some(image) = images.get(image_handle) {
            match background.anchor {
                Background::ANCHOR_TOP => {
                    transform.translation.y =  game_extents.0.y - image.size_f32().y * 0.5 * background.pivot.y;
                },
                Background::ANCHOR_BOTTOM => {
                    transform.translation.y =  image.size_f32().y * 0.5 * background.pivot.y - game_extents.0.y;
                },
                _ => {},
            }
        }
    });
}