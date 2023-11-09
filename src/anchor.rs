use bevy::{prelude::*, sprite::Anchor};

use crate::GameExtents;

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
    game_extents: Res<GameExtents>,
    materials: Res<Assets<ColorMaterial>>,
    images: Res<Assets<Image>>,
) {
    query.for_each_mut(|(mut transform, material_handle, anchored_sprite)| {
        let material = materials.get(material_handle).unwrap();
        let image_handle = material.texture.clone().unwrap();
        if let Some(image) = images.get(image_handle) {
            transform.translation = (game_extents.0 * 2.0 * anchored_sprite.position.as_vec() 
                                    + image.size_f32() * (anchored_sprite.pivot.as_vec())).extend(transform.translation.z);
            // match anchored_sprite.position {
            //     Anchor::TopCenter => {
            //         transform.translation.y =  game_extents.0.y - image.size_f32().y * anchored_sprite.pivot.as_vec().y;
            //     },
            //     Anchor::BottomCenter => {
            //         transform.translation.y =  image.size_f32().y * anchored_sprite.pivot.as_vec().y  - game_extents.0.y;
            //     },
            //     _ => {},
            // }
        }
    });
}