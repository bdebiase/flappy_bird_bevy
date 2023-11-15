use bevy::{app::PluginGroupBuilder, prelude::*, render::camera::ScalingMode};
use bevy_asset_loader::{
    asset_collection::AssetCollection,
    loading_state::{LoadingState, LoadingStateAppExt},
};
use bevy_camera_shake::{CameraShakePlugin, RandomSource, Shake2d};
use bevy_xpbd_2d::{plugins::PhysicsPlugins, resources::Gravity};
use rand::{thread_rng, Rng};

use crate::{
    anchor::AnchorPlugin,
    animation::AnimationPlugin,
    background::BackgroundPlugin,
    ground::GroundPlugin,
    menu::{MenuPlugin, MenuState},
    networking::NetworkingPlugin,
    pipes::PipesPlugin,
    player::{Player, PlayerPlugin},
    tiling::TilingPlugin,
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
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PhysicsPlugins::default(),
            CameraShakePlugin,
            AnimationPlugin,
            AnchorPlugin,
            TilingPlugin,
            MenuPlugin,
            BackgroundPlugin,
            GroundPlugin,
            PlayerPlugin,
            PipesPlugin,
        ))
        .add_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::Loading).continue_to_state(GameState::Waiting),
        )
        .add_collection_to_loading_state::<_, GameAssets>(GameState::Loading)
        .insert_resource(ClearColor(Color::hex("#4EC0CA").unwrap()))
        .insert_resource(Gravity(-Vec2::Y * 400.0))
        .insert_resource(GameScore(0))
        .insert_resource(GameSpeed(50.0))
        .insert_resource(DistanceTraveled(0.0))
        .insert_resource(GameBoundaries::default())
        .add_systems(Startup, setup)
        .add_systems(OnEnter(GameState::Waiting), restart)
        .add_systems(PreUpdate, update_boundaries)
        .add_systems(PreUpdate, update_distance)
        .add_systems(PreUpdate, update_camera)
        .add_systems(Update, debug);
    }
}

struct Random;

impl RandomSource for Random {
    fn rand(&self, _time: f32) -> f32 {
        random_number()
    }
}

fn random_number() -> f32 {
    let mut rng = thread_rng();
    let x: f32 = rng.gen();
    x * 2.0 - 1.0
}

fn setup(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scale *= 0.25;
    // camera_bundle.transform.translation.x += 50.0;
    commands
        .spawn((
            SpatialBundle::default(),
            Shake2d {
                max_offset: Vec2::new(90.0, 45.0),
                max_roll: 0.2,
                trauma: 0.0,
                trauma_power: 3.0,
                decay: 0.75,
                random_sources: [Box::new(Random), Box::new(Random), Box::new(Random)],
            },
        ))
        .with_children(|parent| {
            parent.spawn(camera_bundle);
        });
}

fn restart(
    mut distance_traveled: ResMut<DistanceTraveled>,
    mut game_score: ResMut<GameScore>,
    mut next_menu_state: ResMut<State<MenuState>>,
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
    let (transform, projection) = camera_query.single();
    let view_extents = Vec2::new(projection.area.width(), projection.area.height()) * 0.5
        + transform.translation.truncate();
    let game_height = 175.0;
    game_boundaries.min = Vec2::new(-view_extents.x * 1.25, -50.0);
    game_boundaries.max = Vec2::new(view_extents.x * 1.25, game_boundaries.min.y + game_height);
    // max: Vec2::new(view_extents.x * 1.25, view_extents.y - 20.0),
}

fn update_camera(
    mut query: Query<(&mut Transform, &OrthographicProjection), With<Camera>>,
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
    game_boundaries: Res<GameBoundaries>,
) {
    query.for_each_mut(|(mut transform, projection)| {
        let width = game_boundaries.max.x * 0.5 - game_boundaries.min.x * 0.5;
        transform.translation.x = width * 0.25;

        player_query.for_each(|player_transform| {
            // if projection.area.size().y  > game_boundaries.size().y {
            //     transform.translation.y = max_clamp;
            //     return;
            // }

            if player_transform.translation.y
                > transform.translation.y + projection.area.size().y * 0.25
            {
                transform.translation.y =
                    player_transform.translation.y - projection.area.size().y * 0.25;
            } else if player_transform.translation.y < transform.translation.y {
                transform.translation.y = player_transform.translation.y;
            }

            let min_clamp = game_boundaries.min.y + projection.area.half_size().y * 0.75;
            let max_clamp = game_boundaries.max.y - projection.area.half_size().y * 0.75;

            if min_clamp > max_clamp {
                transform.translation.y =
                    game_boundaries.size().y * 0.5 - projection.area.half_size().y * 0.5;
                return;
            }

            if transform.translation.y > max_clamp {
                transform.translation.y = max_clamp;
            } else if transform.translation.y < min_clamp {
                transform.translation.y = min_clamp;
            }
        });
    });
}

fn debug(mut gizmos: Gizmos, camera_query: Query<(&Transform, &OrthographicProjection)>) {
    camera_query.for_each(|(transform, projection)| {});
}
