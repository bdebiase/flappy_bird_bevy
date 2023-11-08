mod physics;
mod background;
mod player;
mod tiling;
mod ground;
mod stacking;

use bevy::{prelude::*, asset::LoadedFolder, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};
use ground::GroundPlugin;
use physics::{PhysicsPlugin, Velocity};
use background::{BackgroundPlugin, Background};
use stacking::StackingPlugin;
use player::PlayerPlugin;
use tiling::TilingPlugin;

#[derive(Component)]
struct Debug;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
enum GameState {
    #[default]
    Loading,
    Finished,
}

#[derive(Resource)]
struct DistanceTraveled(f32);

#[derive(Resource)]
struct BaseHeight(f32);

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
            TilingPlugin,
            // StackingPlugin,
            PhysicsPlugin,
            BackgroundPlugin,
            GroundPlugin,
            PlayerPlugin,
        ))
        .insert_resource(ClearColor(Color::hex("#4EC0CA").unwrap()))
        .insert_resource(Velocity::from(Vec2::new(150.0, 0.0)))
        .insert_resource(DistanceTraveled(0.0))
        .insert_resource(BaseHeight(0.0))
        .add_systems(Startup, setup)
        .add_systems(Update, (
            load_assets.run_if(in_state(GameState::Loading)),
        ))
        .add_systems(PreUpdate, update_base_height)
        .add_systems(PostUpdate, update_distance)
        .add_systems(PostUpdate, update_debug)
        .run();
}

fn setup(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scale *= 0.25;
    commands.spawn(camera_bundle);

    commands.spawn((SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(10.0, 10.0)),
            ..default()
        },
        ..default()
    }, Debug));
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
            },
            AssetEvent::Added { id } => {
                println!("Added asset with id: {}", id);
                *assets_loaded += 1;
            },
            _ => {},
        }
    }

    if *assets_loaded == *assets_to_load && *assets_to_load != 0 {
        next_state.set(GameState::Finished);
    }
}

fn update_distance(
    mut distance_traveled: ResMut<DistanceTraveled>,
    velocity: Res<Velocity>,
    time: Res<Time>,
) {
    distance_traveled.0 += velocity.x * time.delta_seconds();
}

fn update_base_height(
    mut base_height: ResMut<BaseHeight>,
    projection_query: Query<&OrthographicProjection>,
    windows: Query<&Window>,
) {
    let primary_window = windows.single();
    let projection: &OrthographicProjection = projection_query.single();

    let window_size = Vec2::new(primary_window.width(), primary_window.height()) * projection.scale;
    let offset = -window_size * 0.5 * 0.125;

    base_height.0 = -(window_size.y * 0.5 * 0.5) + offset.y;
}

fn update_debug(
    mut query: Query<&mut Transform, With<Debug>>,
    base_height: Res<BaseHeight>,
) {
    query.for_each_mut(|mut transform| {
        transform.translation.y = base_height.0;
    });
}