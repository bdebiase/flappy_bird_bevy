use bevy::{app::PluginGroupBuilder, asset::LoadedFolder, prelude::*};
use bevy_asset_loader::{
    asset_collection::AssetCollection,
    loading_state::{LoadingState, LoadingStateAppExt},
};

use crate::{
    anchor::AnchorPlugin,
    background::BackgroundPlugin,
    ground::GroundPlugin,
    physics::{PhysicsPlugin, Velocity},
    pipes::PipesPlugin,
    player::PlayerPlugin,
    tiling::TilingPlugin,
};

#[derive(Component)]
struct Debug;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
pub enum GameState {
    #[default]
    Loading,
    Idling,
    Playing,
    Dead,
}

#[derive(Resource, Deref, DerefMut)]
pub struct DistanceTraveled(pub f32);

#[derive(Resource)]
pub struct GameSettings {
    pub scaling: f32,
}

#[derive(Resource, Default, Deref, DerefMut)]
pub struct GameBoundaries(Rect);

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(AnchorPlugin)
            .add(TilingPlugin)
            .add(PhysicsPlugin)
            .add(GamePlugin)
            .add(BackgroundPlugin)
            .add(GroundPlugin)
            .add(PlayerPlugin)
            .add(PipesPlugin)
    }
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_loading_state(
                LoadingState::new(GameState::Loading).continue_to_state(GameState::Idling),
            )
            .insert_resource(ClearColor(Color::hex("#4EC0CA").unwrap()))
            .insert_resource(Velocity::from(Vec2::new(50.0, 0.0)))
            .insert_resource(DistanceTraveled(0.0))
            .insert_resource(GameSettings { scaling: 0.25 })
            .insert_resource(GameBoundaries::default())
            .add_systems(Startup, setup)
            .add_systems(PreUpdate, update_boundaries)
            .add_systems(PostUpdate, update_distance)
            .add_systems(PostUpdate, update_debug);
    }
}

fn setup(mut commands: Commands, game_settings: Res<GameSettings>) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scale *= game_settings.scaling;
    commands.spawn(camera_bundle);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(10.0, 10.0)),
                ..default()
            },
            ..default()
        },
        Debug,
    ));
}

fn update_distance(
    mut distance_traveled: ResMut<DistanceTraveled>,
    game_state: Res<State<GameState>>,
    velocity: Res<Velocity>,
    time: Res<Time>,
) {
    if *game_state == GameState::Dead {
        return;
    }
    distance_traveled.0 += velocity.x * time.delta_seconds();
}

fn update_boundaries(
    mut game_boundaries: ResMut<GameBoundaries>,
    game_settings: Res<GameSettings>,
    windows: Query<&Window>,
) {
    let primary_window = windows.single();
    let window_size =
        Vec2::new(primary_window.width(), primary_window.height()) * game_settings.scaling;
    let window_extents = window_size * 0.5;
    game_boundaries.0 = Rect {
        min: Vec2::new(-window_extents.x, -window_extents.y * 0.5),
        max: Vec2::new(window_extents.x, window_extents.y - 20.0),
    };
}

fn update_debug(
    mut query: Query<&mut Transform, With<Debug>>,
    game_boundaries: Res<GameBoundaries>,
) {
    query.for_each_mut(|mut transform| {
        transform.translation.y = game_boundaries.0.max.y;
    });
}
