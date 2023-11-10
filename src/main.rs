mod anchor;
mod background;
mod ground;
mod physics;
mod player;
mod tiling;

use anchor::AnchorPlugin;
use background::BackgroundPlugin;
use bevy::{asset::LoadedFolder, prelude::*};
use ground::GroundPlugin;
use physics::{PhysicsPlugin, Velocity};
use player::PlayerPlugin;
use tiling::TilingPlugin;

#[derive(Component)]
struct Debug;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
enum GameState {
    #[default]
    Loading,
    Idling,
    Playing,
    Dead,
}

#[derive(Resource)]
struct DistanceTraveled(f32);

#[derive(Resource)]
struct GameSettings {
    scaling: f32,
}

#[derive(Resource, Default)]
struct GameBoundaries(Rect);

fn main() {
    App::new()
        .add_state::<GameState>()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Flappy Bird".into(),
                        resolution: (480.0, 720.0).into(),
                        present_mode: bevy::window::PresentMode::AutoVsync,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            AnchorPlugin,
            TilingPlugin,
            PhysicsPlugin,
            BackgroundPlugin,
            GroundPlugin,
            PlayerPlugin,
        ))
        .insert_resource(ClearColor(Color::hex("#4EC0CA").unwrap()))
        .insert_resource(Velocity::from(Vec2::new(50.0, 0.0)))
        .insert_resource(DistanceTraveled(0.0))
        .insert_resource(GameSettings { scaling: 0.25 })
        .insert_resource(GameBoundaries::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (load_assets.run_if(in_state(GameState::Loading)),))
        .add_systems(PreUpdate, update_boundaries)
        .add_systems(PostUpdate, update_distance)
        .add_systems(PostUpdate, update_debug)
        .run();
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

fn load_assets(
    mut events: EventReader<AssetEvent<LoadedFolder>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut assets_to_load: Local<i32>,
    mut assets_loaded: Local<i32>,
) {
    for event in events.read() {
        match event {
            AssetEvent::LoadedWithDependencies { id } => {
                println!("Loaded asset with dependencies with id: {}", id);
                *assets_to_load += 1;
            }
            AssetEvent::Added { id } => {
                println!("Added asset with id: {}", id);
                *assets_loaded += 1;
            }
            _ => {}
        }
    }

    if *assets_loaded == *assets_to_load && *assets_to_load != 0 {
        next_state.set(GameState::Idling);
    }
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
