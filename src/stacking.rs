use bevy::{prelude::*, sprite::Mesh2dHandle};

use crate::BaseHeight;

#[derive(Component, Default)]
pub struct StackedSprite {
    pub order: f32,
}

#[derive(Component)]
struct Parallax {
    ratio: f32,
}


#[derive(Resource)]
struct StackedEntities(Vec<Entity>);

pub struct StackingPlugin;

impl Plugin for StackingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(StackedEntities(Vec::new()))
            .add_systems(PostUpdate, update_stacking);
    }
}

fn update_stacking(
    mut query: Query<(&mut Transform, &Handle<ColorMaterial>, &StackedSprite)>,
    materials: Res<Assets<ColorMaterial>>,
    images: Res<Assets<Image>>,
    base_height: Res<BaseHeight>,
) {
    let mut stacked_sprites = query.iter_mut().collect::<Vec<_>>();
    stacked_sprites.sort_by(|a, b| a.2.order.partial_cmp(&b.2.order).unwrap());

    let mut cumulative_height = base_height.0;
    for (mut transform, material_handle, stacked_sprite) in stacked_sprites {
        let material = materials.get(material_handle).unwrap();
        let image_handle = material.texture.clone().unwrap();
        if let Some(image) = images.get(image_handle) {
            // if stacked_sprite.order == 0.0 {
            //     transform.translation.y = cumulative_height - image.size_f32().y / 2.0;
            // } else {
            //     transform.translation.y = cumulative_height + image.size_f32().y / 2.0;
            // }
            // cumulative_height = transform.translation.y + image.size_f32().y / 2.0;
            // println!("{}", cumulative_height);
        }
    };
}