use bevy::{
    prelude::*,
    render::texture::{ImageAddressMode, ImageSampler, ImageSamplerDescriptor},
    sprite::Mesh2dHandle,
};

use crate::game::DistanceTraveled;

#[derive(Component, Default)]
pub struct Tiling {
    pub tile_x: bool,
    pub tile_y: bool,
    pub uv_offset: Vec2,
}

#[derive(Component)]
pub struct Parallax {
    pub ratio: f32,
}

impl Default for Parallax {
    fn default() -> Self {
        Self { ratio: 1.0 }
    }
}

pub struct TilingPlugin;

impl Plugin for TilingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (tile_textures, apply_uvs, parllax));
    }
}

fn tile_textures(
    mut images: ResMut<Assets<Image>>,
    materials: Res<Assets<ColorMaterial>>,
    query: Query<(&Handle<ColorMaterial>, &Tiling), Changed<Tiling>>,
) {
    query.for_each(|(material_handle, tiling)| {
        let material = materials.get(material_handle).unwrap();
        if let Some(texture_handle) = &material.texture {
            let texture = images.get_mut(texture_handle).unwrap();
            let mut sampler_desc = ImageSamplerDescriptor::default();
            if tiling.tile_x {
                sampler_desc.address_mode_u = ImageAddressMode::Repeat;
            }
            if tiling.tile_y {
                sampler_desc.address_mode_v = ImageAddressMode::Repeat;
            }
            texture.sampler = ImageSampler::Descriptor(sampler_desc.clone());
        }
    });
}

fn apply_uvs(
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<(&Tiling, &Transform, &Mesh2dHandle, &Handle<ColorMaterial>)>,
    materials: Res<Assets<ColorMaterial>>,
    images: Res<Assets<Image>>,
) {
    query.for_each(|(tiling, transform, mesh_handle, material_handle)| {
        let material = materials.get(material_handle).unwrap();
        let image_handle = material.texture.clone().unwrap();
        if let Some(image) = images.get(image_handle) {
            let texture_size = image.size_f32();
            let tile_count = transform.scale.xy() / texture_size;
            let offset: Vec2 = tiling.uv_offset / image.size_f32();
            let uvs = vec![
                [offset.x, offset.y + tile_count.y],
                [offset.x, offset.y],
                [offset.x + tile_count.x, offset.y],
                [offset.x + tile_count.x, offset.y + tile_count.y],
            ];

            let mesh = meshes.get_mut(mesh_handle.0.clone()).unwrap();
            mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        }
    });
}

fn parllax(mut query: Query<(&mut Tiling, &Parallax)>, distance_traveled: Res<DistanceTraveled>) {
    query.for_each_mut(|(mut tiling, parallax)| {
        tiling.uv_offset.x = **distance_traveled * parallax.ratio;
    });
}
