use bevy::prelude::*;
use bevy_asset_loader::{asset_collection::AssetCollection, loading_state::LoadingStateAppExt};
use rand::Rng;

use crate::{
    game::{DistanceTraveled, GameAssets, GameBoundaries, GameState},
    physics::Collider,
};

#[derive(Event, Default)]
pub struct PipeSpawnEvent {
    position: Vec2,
    gap_spacing: f32,
}

#[derive(Component)]
pub struct PipeSpawner {
    distance_spacing: f32,
    next_position: Vec2,
}

impl Default for PipeSpawner {
    fn default() -> Self {
        Self {
            distance_spacing: 75.0,
            next_position: Vec2::ZERO,
        }
    }
}

#[derive(Component, Default)]
pub struct Pipes {
    spawn_position: Vec2,
}

#[derive(Component)]
pub struct Pipe;

#[derive(Component)]
pub struct PipeArea;

pub struct PipesPlugin;

impl Plugin for PipesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PipeSpawnEvent>()
            .add_systems(OnExit(GameState::Loading), setup)
            .add_systems(
                Update,
                (spawner, handle_spawning, handle_despawning, move_pipes)
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnEnter(GameState::Waiting), restart);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((SpatialBundle::default(), PipeSpawner::default()));
}

fn restart(
    mut commands: Commands,
    mut spawner_query: Query<&mut PipeSpawner>,
    query: Query<Entity, With<Pipes>>,
) {
    query.for_each(|entity| {
        commands.entity(entity).despawn_recursive();
    });

    spawner_query.for_each_mut(|mut spawner| {
        spawner.next_position = Vec2::ZERO;
    });
}

fn spawner(
    mut event_writer: EventWriter<PipeSpawnEvent>,
    mut query: Query<(&mut PipeSpawner, &mut Transform)>,
    game_boundaries: Res<GameBoundaries>,
    distance_traveled: Res<DistanceTraveled>,
) {
    query.for_each_mut(|(mut spawner, mut transform)| {
        if **distance_traveled >= spawner.next_position.x {
            let mut rng = rand::thread_rng();
            let spacing = 48.0;
            transform.translation = Vec3::X * game_boundaries.max.x
                + Vec3::Y
                    * rng.gen_range(
                        game_boundaries.min.y + spacing * 0.5 + 14.0
                            ..=game_boundaries.max.y - spacing * 0.5 - 16.0 * 2.0,
                    );
            event_writer.send(PipeSpawnEvent {
                position: transform.translation.truncate(),
                gap_spacing: spacing,
            });
            spawner.next_position.x = **distance_traveled + spawner.distance_spacing;
        }
    });
}

fn handle_spawning(
    mut commands: Commands,
    mut event_reader: EventReader<PipeSpawnEvent>,
    distance_traveled: Res<DistanceTraveled>,
    game_assets: Res<GameAssets>,
    images: Res<Assets<Image>>,
) {
    for event in event_reader.read() {
        let image = images.get(game_assets.pipe_image.clone()).unwrap();
        let pipe_offset = Vec3::Y * (event.gap_spacing * 0.5 + image.size_f32().y * 0.5);
        commands
            .spawn((
                SpatialBundle {
                    transform: Transform::from_translation(event.position.extend(-10.0)),
                    ..default()
                },
                Pipes {
                    spawn_position: event.position + Vec2::X * **distance_traveled,
                    ..default()
                },
            ))
            .with_children(|parent| {
                parent.spawn((
                    SpriteBundle {
                        texture: game_assets.pipe_image.clone(),
                        transform: Transform::from_translation(-pipe_offset),
                        ..default()
                    },
                    Collider {
                        size: image.size_f32(),
                    },
                    Pipe,
                ));

                parent.spawn((
                    SpriteBundle {
                        texture: game_assets.pipe_image.clone(),
                        transform: Transform::from_scale(Vec3::new(1.0, -1.0, 1.0))
                            .with_translation(pipe_offset),
                        ..default()
                    },
                    Collider {
                        size: image.size_f32(),
                    },
                    Pipe,
                ));

                parent.spawn((
                    SpatialBundle::default(),
                    Collider {
                        size: Vec2::new(1.0, event.gap_spacing),
                    },
                    PipeArea,
                ));
            });
    }
}

fn handle_despawning(
    mut commands: Commands,
    query: Query<(&Transform, Entity), With<Pipes>>,
    game_boundaries: Res<GameBoundaries>,
) {
    query.for_each(|(transform, entity)| {
        if transform.translation.x < game_boundaries.min.x {
            commands.entity(entity).despawn_recursive();
        }
    });
}

fn move_pipes(
    mut query: Query<(&mut Transform, &Pipes)>,
    distance_traveled: Res<DistanceTraveled>,
) {
    query.for_each_mut(|(mut transform, pipes)| {
        let distance = pipes.spawn_position.x - **distance_traveled;
        transform.translation.x = distance;
    });
}
