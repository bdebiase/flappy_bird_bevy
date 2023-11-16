use bevy::prelude::*;
use bevy_camera_shake::{RandomSource, Shake2d, CameraShakePlugin};
use rand::{thread_rng, Rng};

use crate::{game::{GameBoundaries, GameState}, player::Player};

pub struct GameCameraPlugin;

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

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CameraShakePlugin)
            .add_systems(Startup, setup)
            .add_systems(PreUpdate, update_camera);
    }
}

fn setup(
    mut commands: Commands,
) {
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

fn update_camera(
    mut query: Query<(&mut Transform, &OrthographicProjection), With<Camera>>,
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
    game_boundaries: Res<GameBoundaries>,
) {
    query.for_each_mut(|(mut transform, projection)| {
        let width = game_boundaries.max.x * 0.5 - game_boundaries.min.x * 0.5;
        transform.translation.x = width * 0.25;

        player_query.for_each(|player_transform| {
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