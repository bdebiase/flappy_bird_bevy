use bevy::{prelude::*, sprite::Anchor};

use crate::GameBoundaries;

#[derive(Component, Default)]
pub struct AnchoredSprite {
    pub position: Anchor,
    pub pivot: Anchor,
}

pub struct AnchorPlugin;

impl Plugin for AnchorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, reposition);
    }
}

fn reposition(
    mut query: Query<(&mut Transform, &Handle<ColorMaterial>, &AnchoredSprite)>,
    game_boundaries: Res<GameBoundaries>,
    materials: Res<Assets<ColorMaterial>>,
    images: Res<Assets<Image>>,
) {
    query.for_each_mut(|(mut transform, material_handle, anchored_sprite)| {
        let material = materials.get(material_handle).unwrap();
        let image_handle = material.texture.clone().unwrap();
        if let Some(image) = images.get(image_handle) {
            let game_size = game_boundaries.0.max - game_boundaries.0.min;
            let anchor_position = game_boundaries.0.min + game_size * (anchored_sprite.position.as_vec() + Vec2::new(0.5, 0.5));
            let pivot_offset = image.size_f32() * anchored_sprite.pivot.as_vec();
            println!("{:?}", game_boundaries.0);

            transform.translation = (anchor_position + pivot_offset).extend(transform.translation.z);
        }
    });
}