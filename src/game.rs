use bevy::{app::PluginGroupBuilder, prelude::*};
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};

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

#[derive(Resource, Deref, DerefMut)]
pub struct GameSpeed(f32);

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
            .insert_resource(GameSpeed(50.0))
            .insert_resource(DistanceTraveled(0.0))
            .insert_resource(GameBoundaries::default())
            .add_systems(Startup, setup)
            .add_systems(PreUpdate, update_boundaries)
            .add_systems(PostUpdate, update_distance)
            .add_systems(PostUpdate, update_debug);
    }
}

fn setup(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scale *= 0.25;
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
    game_speed: Res<GameSpeed>,
    time: Res<Time>,
) {
    if *game_state == GameState::Dead {
        return;
    }
    distance_traveled.0 += game_speed.0 * time.delta_seconds();
}

fn update_boundaries(
    mut game_boundaries: ResMut<GameBoundaries>,
    projection_query: Query<&OrthographicProjection>,
) {
    let projection = projection_query.single();
    let view_extents = Vec2::new(projection.area.width(), projection.area.height()) * 0.5;
    game_boundaries.0 = Rect {
        min: Vec2::new(-view_extents.x, -view_extents.y * 0.5),
        max: Vec2::new(view_extents.x, view_extents.y - 20.0),
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
