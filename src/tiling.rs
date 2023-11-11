use bevy::{
    prelude::*,
    render::texture::{ImageAddressMode, ImageSampler, ImageSamplerDescriptor},
    sprite::Mesh2dHandle,
};

use crate::game::DistanceTraveled;

#[derive(Component, Default)]
pub struct Tiling {
    pub uv_offset: Vec2,
}

#[derive(Component)]
pub struct Parallax {
    pub ratio: f32,
}

impl Default for Parallax {
    fn default() -> Self {
        Self {
            ratio: 1.0,
        }
    }
}

pub struct TilingPlugin;

impl Plugin for TilingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (tile_textures, resize_tiling, scaling, parllax));
    }
}

fn tile_textures(
    mut images: ResMut<Assets<Image>>,
    materials: Res<Assets<ColorMaterial>>,
    query: Query<&Handle<ColorMaterial>, (With<Tiling>, Added<Handle<ColorMaterial>>)>,
) {
    query.for_each(|material_handle| {
        let material = materials.get(material_handle).unwrap();
        if let Some(texture_handle) = &material.texture {
            let texture = images.get_mut(texture_handle).unwrap();
            let sampler_desc = ImageSamplerDescriptor {
                address_mode_u: ImageAddressMode::Repeat,
                address_mode_v: ImageAddressMode::Repeat,
                ..Default::default()
            };
            texture.sampler = ImageSampler::Descriptor(sampler_desc.clone());
        }
    });
}

fn resize_tiling(
    mut query: Query<(
        &mut Tiling,
        &Mesh2dHandle,
        &Handle<ColorMaterial>,
    )>,
    mut meshes: ResMut<Assets<Mesh>>,
    projection_query: Query<&OrthographicProjection>,
    materials: Res<Assets<ColorMaterial>>,
    images: Res<Assets<Image>>,
) {
    let projection = projection_query.single();
    let view_size = Vec2::new(projection.area.width(), projection.area.height());
    query.for_each_mut(|(tiling, mesh_handle, material_handle)| {
        let material = materials.get(material_handle).unwrap();
        let image_handle = material.texture.clone().unwrap();
        if let Some(image) = images.get(image_handle) {
            let texture_size = image.size_f32();
            let tile_count_x = view_size.x / texture_size.x;
            let offset = tiling.uv_offset / image.size_f32();
            let uvs = vec![
                [offset.x, offset.y + 1.0],
                [offset.x, offset.y],
                [offset.x + tile_count_x, offset.y],
                [offset.x + tile_count_x, offset.y + 1.0],
            ];

            let mesh = meshes.get_mut(mesh_handle.0.clone()).unwrap();
            mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        }
    });
}

fn scaling(
    mut query: Query<(&mut Transform, &Handle<ColorMaterial>), With<Tiling>>,
    projection_query: Query<&OrthographicProjection>,
    materials: Res<Assets<ColorMaterial>>,
    images: Res<Assets<Image>>,
) {
    let projection = projection_query.single();
    let view_size = Vec2::new(projection.area.width(), projection.area.height());
    query.for_each_mut(|(mut transform, material_handle)| {
        let material = materials.get(material_handle).unwrap();
        let image_handle = material.texture.clone().unwrap();
        if let Some(image) = images.get(image_handle) {
            transform.scale.x = view_size.x;
            transform.scale.y = image.size_f32().y;
        }
    });
}

fn parllax(
    mut query: Query<(&mut Tiling, &Parallax)>,
    distance_traveled: Res<DistanceTraveled>,
) {
    query.for_each_mut(|(mut tiling, parallax)| {
        tiling.uv_offset.x = distance_traveled.0 * parallax.ratio;
    });
}