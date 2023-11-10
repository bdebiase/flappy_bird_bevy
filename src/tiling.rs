use bevy::{
    prelude::*,
    sprite::Mesh2dHandle,
};

use crate::{DistanceTraveled, GameSettings};

#[derive(Component)]
pub struct Tiling {
    pub parallax_ratio: f32,
}

impl Default for Tiling {
    fn default() -> Self {
        Self {
            parallax_ratio: 1.0,
        }
    }
}

pub struct TilingPlugin;

impl Plugin for TilingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, resize_tiling);
    }
}

fn resize_tiling(
    mut query: Query<(
        &mut Transform,
        &Mesh2dHandle,
        &Handle<ColorMaterial>,
        &Tiling,
    )>,
    mut meshes: ResMut<Assets<Mesh>>,
    windows: Query<&Window>,
    materials: Res<Assets<ColorMaterial>>,
    images: Res<Assets<Image>>,
    game_settings: Res<GameSettings>,
    distance_traveled: Res<DistanceTraveled>,
) {
    query.for_each_mut(|(mut transform, mesh_handle, material_handle, tiling)| {
        let material = materials.get(material_handle).unwrap();
        let image_handle = material.texture.clone().unwrap();
        if let Some(image) = images.get(image_handle) {
            let primary_window = windows.single();
            let window_size =
                Vec2::new(primary_window.width(), primary_window.height()) * game_settings.scaling;
            let texture_size = image.size_f32();
            let texture_aspect_ratio = texture_size.y / texture_size.x;

            let tile_count_x = window_size.x / texture_size.x;
            let uv_pan = distance_traveled.0 * tiling.parallax_ratio / texture_size.x;
            let uvs: Vec<[f32; 2]> = vec![
                [uv_pan, 1.0],
                [uv_pan, 0.0],
                [uv_pan + tile_count_x, 0.0],
                [uv_pan + tile_count_x, 1.0],
            ];

            let mesh = meshes.get_mut(mesh_handle.0.clone()).unwrap();
            mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

            let scale_x = window_size.x / tile_count_x;
            transform.scale.x = window_size.x;
            transform.scale.y = scale_x * texture_aspect_ratio;
        }
    });
}
