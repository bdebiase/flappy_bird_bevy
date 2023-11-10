use bevy::prelude::*;
use bevy_asset_loader::{asset_collection::AssetCollection, loading_state::LoadingStateAppExt};

use crate::game::{GameBoundaries, GameState};

#[derive(Event, Default)]
pub struct PipeSpawnEvent;

pub struct Pipe;

#[derive(AssetCollection, Resource)]
struct PipeAssets {
    #[asset(path = "sprites/pipe.png")]
    image: Handle<Image>,
}

pub struct PipesPlugin;

impl Plugin for PipesPlugin {
    fn build(&self, app: &mut App) {
        app.add_collection_to_loading_state::<_, PipeAssets>(GameState::Loading)
            .add_event::<PipeSpawnEvent>();
        // .add_systems(Update, spawner);
    }
}

fn spawner(
    mut commands: Commands,
    mut event_reader: EventReader<PipeSpawnEvent>,
    game_boundaries: Res<GameBoundaries>,
    pipe_assets: Res<PipeAssets>,
) {
    for event in event_reader.read() {
        commands
            .spawn(SpatialBundle {
                transform: Transform::from_translation(Vec3::X * game_boundaries.max.x),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(SpriteBundle {
                    texture: pipe_assets.image.clone(),
                    ..default()
                });
            });
    }
}
