use bevy::prelude::*;
use bevy_asset_loader::{asset_collection::AssetCollection, loading_state::LoadingStateAppExt};

use crate::{
    game::{GameBoundaries, GameState, GameSpeed},
    physics::Velocity,
};

#[derive(Component, Default)]
pub struct Player;

#[derive(AssetCollection, Resource)]
struct PlayerAssets {
    #[asset(path = "sprites/bird", collection(typed))]
    folder: Vec<Handle<Image>>,
}

#[derive(Component, Default)]
pub struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Default, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(Resource)]
struct FlapForce(f32);

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    pub player: Player,
    pub animation_indices: AnimationIndices,
    pub animation_timer: AnimationTimer,
    pub velocity: Velocity,

    #[bundle()]
    pub sprite_sheet_bundle: SpriteSheetBundle,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_collection_to_loading_state::<_, PlayerAssets>(GameState::Loading)
            .insert_resource(FlapForce(100.0))
            .add_systems(OnExit(GameState::Loading), setup)
            .add_systems(
                Update,
                (
                    auto_flap.run_if(in_state(GameState::Idling)),
                    flap_input
                        .run_if(can_flap)
                        .run_if(not(in_state(GameState::Dead))),
                    animate_sprite,
                    animate_velocity,
                )
                    .chain(),
            )
            .add_systems(PostUpdate, bounds_collision);
    }
}

fn setup(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
    player_assets: Res<PlayerAssets>,
) {
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    for handle in player_assets.folder.iter() {
        let id = handle.id();
        let Some(texture) = textures.get(id) else {
            warn!(
                "{:?} did not resolve to an `Image` asset.",
                handle.path().unwrap()
            );
            continue;
        };

        texture_atlas_builder.add_texture(id, texture);
    }

    let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
    let vendor_handle = player_assets.folder.get(0).unwrap();
    let vendor_index = texture_atlas.get_texture_index(vendor_handle).unwrap();
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn(PlayerBundle {
        sprite_sheet_bundle: SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(vendor_index),
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_translation(Vec3::Z * 500.0),
            ..default()
        },
        animation_indices: AnimationIndices { first: 0, last: 2 },
        animation_timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        ..default()
    });
}

fn animate_sprite(
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &AnimationIndices,
    )>,
    time: Res<Time>,
) {
    query.for_each_mut(|(mut timer, mut sprite, indices)| {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    });
}

fn can_flap(query: Query<&Transform, With<Player>>, game_boundaries: Res<GameBoundaries>) -> bool {
    for transform in query.iter() {
        if transform.translation.y > game_boundaries.max.y {
            return false;
        }
    }
    return true;
}

fn flap_input(
    mut query: Query<&mut Velocity, With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
    game_state: Res<State<GameState>>,
    keyboard_input: Res<Input<KeyCode>>,
    flap_force: Res<FlapForce>,
) {
    query.for_each_mut(|mut velocity| {
        if keyboard_input.just_pressed(KeyCode::Space) {
            velocity.y = flap_force.0;
    
            if *game_state == GameState::Idling {
                next_state.set(GameState::Playing);
            }
        }
    });
}

fn auto_flap(
    mut query: Query<(&mut Velocity, &Transform), With<Player>>,
    flap_force: Res<FlapForce>,
) {
    query.for_each_mut(|(mut velocity, transform)| {
        if transform.translation.y < -0.0 && velocity.y < 0.0 {
            velocity.y = flap_force.0;
        }
    });
}

fn animate_velocity(
    mut query: Query<(&mut Transform, &Velocity), With<Player>>,
    game_speed: Res<GameSpeed>,
    time: Res<Time>,
) {
    query.for_each_mut(|(mut transform, velocity)| {
        transform.rotation = transform.rotation.lerp(
            Quat::from_rotation_z(velocity.y.atan2(**game_speed)),
            25.0 * time.delta_seconds(),
        );
    });
}

fn bounds_collision(
    mut query: Query<(&mut Transform, &mut Velocity), With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
    game_boundaries: Res<GameBoundaries>,
) {
    query.for_each_mut(|(mut transform, mut velocity)| {
        if transform.translation.y < game_boundaries.min.y {
            transform.translation.y = game_boundaries.min.y;
            velocity.0 = Vec2::ZERO;

            next_state.set(GameState::Dead);
        }
    });
}
