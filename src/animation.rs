use bevy::prelude::*;

#[derive(Asset, TypePath)]
pub struct Animation(pub benimator::Animation);

#[derive(Default, Component, Deref, DerefMut)]
pub struct AnimationState(pub benimator::State);

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<Animation>().add_systems(Update, animate);
    }
}

fn animate(
    mut query: Query<(
        &mut AnimationState,
        &mut TextureAtlasSprite,
        &Handle<Animation>,
    )>,
    animations: Res<Assets<Animation>>,
    time: Res<Time>,
) {
    query.for_each_mut(|(mut animation_state, mut texture_atlas, animation)| {
        animation_state.update(&animations.get(animation).unwrap().0, time.delta());
        texture_atlas.index = animation_state.frame_index();
    });
}

// fn animate(
//     mut query: Query<(
//         &mut AnimationTimer,
//         &mut TextureAtlasSprite,
//         &AnimationIndices,
//     )>,
//     time: Res<Time>,
// ) {
//     query.for_each_mut(|(mut timer, mut sprite, indices)| {
//         timer.tick(time.delta());
//         if timer.just_finished() {
//             sprite.index = if sprite.index == indices.last {
//                 indices.first
//             } else {
//                 sprite.index + 1
//             };
//         }
//     });
// }
