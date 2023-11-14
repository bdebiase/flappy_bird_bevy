use bevy::{prelude::*, sprite::Anchor};

use crate::game::GameBoundaries;

#[derive(Component, Default)]
pub struct AnchoredSprite {
    pub position: Anchor,
    pub pivot: Anchor,
    pub stretch: bool,
}

pub struct AnchorPlugin;

impl Plugin for AnchorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            transform.run_if(resource_changed::<GameBoundaries>()),
        );
    }
}

fn transform(
    mut query: Query<(&mut Transform, &Handle<ColorMaterial>, &AnchoredSprite)>,
    game_boundaries: Res<GameBoundaries>,
    projection_query: Query<(&Transform, &OrthographicProjection), Without<AnchoredSprite>>,
    materials: Res<Assets<ColorMaterial>>,
    images: Res<Assets<Image>>,
) {
    query.for_each_mut(|(mut transform, material_handle, anchored_sprite)| {
        let material = materials.get(material_handle).unwrap();
        let image_handle = material.texture.clone().unwrap();
        let (camera_transform, projection) = projection_query.single();
        if let Some(image) = images.get(image_handle) {
            let game_size = game_boundaries.size();
            let anchor_position = game_boundaries.min
                + game_size * (anchored_sprite.position.as_vec() + Vec2::new(0.5, 0.5));
            let pivot_offset = image.size_f32() * anchored_sprite.pivot.as_vec();
            transform.translation =
                (anchor_position + pivot_offset).extend(transform.translation.z);
            transform.scale.x = game_size.x;
            transform.scale.y = image.size_f32().y;
            if anchored_sprite.stretch {
                let min_area = projection.area.min + camera_transform.translation.truncate();
                transform.scale.y = game_boundaries.min.y - min_area.y;
                transform.translation.y = min_area.y + transform.scale.y * 0.5;
            }
        }
    });
}
