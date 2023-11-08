use bevy::{prelude::*, asset::LoadedFolder};

use crate::{physics::Velocity, GameState, GameExtents};

#[derive(Resource, Default)]
struct PlayerSpriteFolder(Handle<LoadedFolder>);

#[derive(Component, Default)]
pub struct Player;

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

    #[bundle()]
    pub sprite_sheet_bundle: SpriteSheetBundle,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FlapForce(100.0))
            .add_systems(OnEnter(GameState::Loading), load_textures)
            .add_systems(OnExit(GameState::Loading), setup)
            .add_systems(Update, (
                auto_flap.run_if(in_state(GameState::Idling)),
                flap_input.run_if(not(in_state(GameState::Dead))),
                translate_player, animate_sprite, animate_velocity
            ).chain())
            .add_systems(PostUpdate, bounds_collision);
    }
}

fn load_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(PlayerSpriteFolder(asset_server.load_folder("sprites/bird")));
}

fn setup(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
    player_sprite_handles: Res<PlayerSpriteFolder>,
    asset_server: Res<AssetServer>,
    loaded_folders: Res<Assets<LoadedFolder>>,
) {
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    let loaded_folder = loaded_folders.get(&player_sprite_handles.0).unwrap();
    for handle in loaded_folder.handles.iter() {
        let id = handle.id().typed_unchecked::<Image>();
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
    let vendor_handle = asset_server.get_handle("sprites/bird/bird0.png").unwrap();
    let vendor_index = texture_atlas.get_texture_index(&vendor_handle).unwrap();
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

fn translate_player(
    mut query: Query<&mut Transform, With<Player>>,
    velocity: Res<Velocity>,
    time: Res<Time>,
) {
    query.for_each_mut(|mut transform| {
        transform.translation.y += velocity.0.y * time.delta_seconds();
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

fn flap_input(
    mut velocity: ResMut<Velocity>,
    mut next_state: ResMut<NextState<GameState>>,
    game_state: Res<State<GameState>>,
    keyboard_input: Res<Input<KeyCode>>,
    flap_force: Res<FlapForce>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        velocity.y = flap_force.0;

        if *game_state == GameState::Idling {
            next_state.set(GameState::Playing);
        }
    }
}

fn auto_flap(
    mut velocity: ResMut<Velocity>,
    query: Query<&Transform, With<Player>>,
    flap_force: Res<FlapForce>,
) {
    query.for_each(|transform| {
        if transform.translation.y < -0.0 && velocity.y < 0.0 {
            velocity.y = flap_force.0;
        }
    })
}

fn animate_velocity(mut query: Query<&mut Transform, With<Player>>, velocity: Res<Velocity>, time: Res<Time>) {
    query.for_each_mut(|mut transform| {
        transform.rotation = transform.rotation.lerp(Quat::from_rotation_z(velocity.y.atan2(velocity.x)), 25.0 * time.delta_seconds());
    });
}

fn bounds_collision(
    mut query: Query<&mut Transform, With<Player>>,
    mut velocity: ResMut<Velocity>,
    mut next_state: ResMut<NextState<GameState>>,
    windows: Query<&Window>,
    projection_query: Query<&OrthographicProjection>,
    game_extents: Res<GameExtents>,
) {
    query.for_each_mut(|mut transform| {
        let primary_window = windows.single();
        let projection = projection_query.single();
        if transform.translation.y < -game_extents.0.y {
            transform.translation.y = -game_extents.0.y;
            velocity.0 = Vec2::ZERO;

            next_state.set(GameState::Dead);
        } else if transform.translation.y > primary_window.height() * 0.5 * projection.scale {
            transform.translation.y = primary_window.height() * 0.5 * projection.scale;
            velocity.y = 0.0;
        }
    });
}