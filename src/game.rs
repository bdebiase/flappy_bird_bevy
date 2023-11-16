use bevy::{app::PluginGroupBuilder, prelude::*};
use bevy_asset_loader::{
    asset_collection::AssetCollection,
    loading_state::{LoadingState, LoadingStateAppExt},
};

use crate::{
    anchor::AnchorPlugin,
    animation::AnimationPlugin,
    level::LevelPlugin,
    menu::MenuPlugin,
    physics::{Gravity, PhysicsPlugin},
    pipes::PipesPlugin,
    player::PlayerPlugin,
    tiling::TilingPlugin, camera::GameCameraPlugin,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
pub enum GameState {
    #[default]
    Loading,
    Waiting,
    Playing,
    Stopped,
    Dead,
}

#[derive(Resource, Deref, DerefMut)]
pub struct DistanceTraveled(pub f32);

#[derive(Resource, Deref, DerefMut)]
pub struct GameScore(i32);

#[derive(Resource, Deref, DerefMut)]
pub struct GameSpeed(f32);

#[derive(Resource, Default, Deref, DerefMut)]
pub struct GameBoundaries(Rect);

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "sprites/ground.png")]
    pub ground_image: Handle<Image>,
    #[asset(path = "sprites/pipe.png")]
    pub pipe_image: Handle<Image>,
    #[asset(path = "sprites/background/mountains.png")]
    pub mountains_image: Handle<Image>,
    #[asset(path = "sprites/background/buildings.png")]
    pub buildings_image: Handle<Image>,
    #[asset(path = "sprites/background/clouds.png")]
    pub clouds_image: Handle<Image>,
    #[asset(path = "sprites/bird", collection(typed))]
    pub player_sprite_folder: Vec<Handle<Image>>,
    #[asset(path = "sprites/ui_background.png")]
    pub ui_background: Handle<Image>,
    #[asset(path = "audio/flap.ogg")]
    pub flap_audio: Handle<AudioSource>,
    #[asset(path = "audio/hit.ogg")]
    pub hit_audio: Handle<AudioSource>,
    #[asset(path = "audio/fall.ogg")]
    pub fall_audio: Handle<AudioSource>,
    #[asset(path = "audio/point.ogg")]
    pub point_audio: Handle<AudioSource>,
}

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(AnimationPlugin)
            .add(AnchorPlugin)
            .add(TilingPlugin)
            .add(PhysicsPlugin)
            // .add(PhysicsDebugPlugin)
            .add(GamePlugin)
            .add(GameCameraPlugin)
            .add(MenuPlugin)
            .add(LevelPlugin)
            .add(PlayerPlugin)
            .add(PipesPlugin)
    }
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_loading_state(
                LoadingState::new(GameState::Loading).continue_to_state(GameState::Waiting),
            )
            .add_collection_to_loading_state::<_, GameAssets>(GameState::Loading)
            .insert_resource(ClearColor(Color::hex("#4EC0CA").unwrap()))
            .insert_resource(Gravity::from(-Vec2::Y * 400.0))
            .insert_resource(GameScore(0))
            .insert_resource(GameSpeed(50.0))
            .insert_resource(DistanceTraveled(0.0))
            .insert_resource(GameBoundaries::default())
            .add_systems(OnEnter(GameState::Waiting), restart)
            .add_systems(PreUpdate, update_boundaries)
            .add_systems(PreUpdate, update_distance);
    }
}

fn restart(
    mut distance_traveled: ResMut<DistanceTraveled>,
    mut game_score: ResMut<GameScore>,
) {
    **distance_traveled = 0.0;
    **game_score = 0;
}

fn update_distance(
    mut distance_traveled: ResMut<DistanceTraveled>,
    game_state: Res<State<GameState>>,
    game_speed: Res<GameSpeed>,
    time: Res<Time>,
) {
    if *game_state == GameState::Stopped || *game_state == GameState::Dead {
        return;
    }
    **distance_traveled += game_speed.0 * time.delta_seconds();
}

fn update_boundaries(
    mut game_boundaries: ResMut<GameBoundaries>,
    camera_query: Query<(&Transform, &OrthographicProjection)>,
) {
    if let Ok((transform, projection)) = camera_query.get_single() {
        let view_extents = Vec2::new(projection.area.width(), projection.area.height()) * 0.5
        + transform.translation.truncate();
        let game_height = 175.0;
        game_boundaries.min = Vec2::new(-view_extents.x * 1.25, -50.0);
        game_boundaries.max = Vec2::new(view_extents.x * 1.25, game_boundaries.min.y + game_height);
        // max: Vec2::new(view_extents.x * 1.25, view_extents.y - 20.0),
    }
}
