use std::time::Duration;

use benimator::{Frame, FrameRate};
use bevy::{
    audio::{PlaybackMode, VolumeLevel},
    input::common_conditions::input_just_pressed,
    prelude::*, sprite::collide_aabb::Collision,
};
use bevy_camera_shake::Shake2d;

use crate::{
    animation::{Animation, AnimationState},
    game::{GameAssets, GameBoundaries, GameScore, GameState},
    physics::{Collider, CollisionEvent, GravityScale, Velocity},
    pipes::{Pipe, PipeArea},
};

#[derive(Component)]
pub struct Player;

#[derive(Resource)]
pub struct FlapForce(pub f32);

#[derive(Resource, Default)]
pub struct PlayerAnimations {
    pub idle: Handle<Animation>,
    pub flap: Handle<Animation>,
    pub dead: Handle<Animation>,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FlapForce(150.0))
            .insert_resource(PlayerAnimations::default())
            .add_systems(Startup, setup_animations)
            .add_systems(OnExit(GameState::Loading), setup)
            .add_systems(OnEnter(GameState::Waiting), restart)
            .add_systems(
                Update,
                (
                    handle_death,
                    auto_flap.run_if(in_state(GameState::Waiting)),
                    flap_input
                        .run_if(can_flap)
                        .run_if(not(in_state(GameState::Stopped)))
                        .run_if(not(in_state(GameState::Dead))),
                    animate_velocity,
                    trigger_restart
                        .run_if(in_state(GameState::Dead))
                        .run_if(input_just_pressed(KeyCode::Space)),
                    collisions,
                )
                    .chain()
                    .run_if(not(in_state(GameState::Loading))),
            );
    }
}

fn setup_animations(
    mut animations: ResMut<Assets<Animation>>,
    mut player_animations: ResMut<PlayerAnimations>,
) {
    let idle_animation =
        Animation(benimator::Animation::from_indices(0..=1, FrameRate::from_fps(1.0)).ping_pong());
    let animation_speed = 50;
    let flap_animation = Animation(
        benimator::Animation::from_frames(vec![
            Frame::new(0, Duration::from_millis(animation_speed)),
            Frame::new(1, Duration::from_millis(animation_speed)),
            Frame::new(2, Duration::from_millis(animation_speed)),
            Frame::new(1, Duration::from_millis(animation_speed)),
            Frame::new(0, Duration::from_millis(animation_speed)),
            Frame::new(1, Duration::from_millis(animation_speed)),
            Frame::new(2, Duration::from_millis(animation_speed)),
            Frame::new(1, Duration::from_millis(animation_speed)),
            Frame::new(0, Duration::from_millis(animation_speed)),
            Frame::new(1, Duration::from_millis(animation_speed)),
            Frame::new(2, Duration::from_millis(animation_speed)),
            Frame::new(1, Duration::from_millis(animation_speed)),
        ])
        .once(),
    );

    let idle_animation_handle = animations.add(idle_animation);
    let flap_animation_handle = animations.add(flap_animation);
    player_animations.idle = idle_animation_handle;
    player_animations.flap = flap_animation_handle;
}

fn setup(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
    game_assets: Res<GameAssets>,
) {
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    for handle in game_assets.player_sprite_folder.iter() {
        let id = handle.id();
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
    let vendor_handle = game_assets.player_sprite_folder.get(0).unwrap();
    let vendor_index = texture_atlas.get_texture_index(vendor_handle).unwrap();
    let vendor_texture = textures.get(vendor_handle).unwrap();
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn((
        SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(vendor_index),
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_translation(Vec3::Z * 500.0),
            ..default()
        },
        // AnimationIndices { first: 0, last: 2 },
        // AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        AnimationState::default(),
        Velocity::default(),
        Collider {
            size: Vec2::new(
                vendor_texture.size_f32().y * 0.9,
                vendor_texture.size_f32().y * 0.8,
            ),
        },
        GravityScale::default(),
        Player,
    ));
}

fn handle_death(
    mut commands: Commands,
    mut query: Query<(&mut Velocity, &mut GravityScale, &Transform), With<Player>>,
    mut death_timer: Local<Timer>,
    game_boundaries: Res<GameBoundaries>,
    game_assets: Res<GameAssets>,
    game_state: Res<State<GameState>>,
    time: Res<Time>,
) {
    if death_timer.duration() == Duration::from_secs_f32(0.0) {
        death_timer.set_duration(Duration::from_secs_f32(1.0));
    }

    if *game_state != GameState::Stopped && *game_state != GameState::Dead {
        if !death_timer.paused() {
            println!("RESET TIMER");
            death_timer.pause();
            death_timer.reset();
        }
        return;
    }

    if death_timer.paused() || death_timer.percent() == 0.0 {
        println!("JUST DIED");
        death_timer.unpause();
        if death_timer.percent() == 0.0 {
            query.for_each_mut(|(mut velocity, mut gravity_scale, _)| {
                **velocity = Vec2::ZERO;
                **gravity_scale = 0.0;
            });    
        }
        death_timer.reset();
    }

    death_timer.tick(time.delta());

    if death_timer.just_finished() {
        query.for_each_mut(|(_, mut gravity_scale, transform)| {
            if transform.translation.y != game_boundaries.min.y {
                **gravity_scale = 1.0;

                commands.spawn(AudioSourceBundle {
                    source: game_assets.fall_audio.clone(),
                    settings: PlaybackSettings {
                        mode: PlaybackMode::Remove,
                        volume: bevy::audio::Volume::Absolute(VolumeLevel::new(0.1)),
                        ..default()
                    },
                });
            }
        });
    }
}

fn restart(
    mut commands: Commands,
    mut query: Query<(&mut Transform,&mut GravityScale, Entity), With<Player>>,
    player_animations: Res<PlayerAnimations>,
) {
    query.for_each_mut(|(mut transform, mut gravity_scale, entity)| {
        transform.translation.x = 0.0;
        transform.translation.y = 0.0;
        transform.rotation = Quat::IDENTITY;
        **gravity_scale = 1.0;

        commands.entity(entity).insert(player_animations.flap.clone());
    });
}

fn trigger_restart(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Waiting);
}

fn can_flap(
    query: Query<(&Transform, &Collider), With<Player>>,
    game_boundaries: Res<GameBoundaries>,
) -> bool {
    for (transform, collider) in query.iter() {
        if transform.translation.y + collider.size.y * 0.5 > game_boundaries.max.y {
            return false;
        }
    }
    return true;
}

fn flap_input(
    mut commands: Commands,
    mut query: Query<(&mut Velocity, &mut AnimationState), With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
    game_assets: Res<GameAssets>,
    game_state: Res<State<GameState>>,
    keyboard_input: Res<Input<KeyCode>>,
    flap_force: Res<FlapForce>,
) {
    query.for_each_mut(
        |(mut velocity, mut animation_state)| {
            if keyboard_input.just_pressed(KeyCode::Space) {
                velocity.y = flap_force.0;
                animation_state.0.reset();

                commands.spawn(AudioSourceBundle {
                    source: game_assets.flap_audio.clone(),
                    settings: PlaybackSettings {
                        mode: PlaybackMode::Remove,
                        volume: bevy::audio::Volume::Absolute(VolumeLevel::new(0.1)),
                        ..default()
                    },
                });

                if *game_state == GameState::Waiting {
                    next_state.set(GameState::Playing);
                }
            }
        },
    );
}

fn auto_flap(
    mut query: Query<(&mut Velocity, &Transform), With<Player>>,
    flap_force: Res<FlapForce>,
) {
    query.for_each_mut(|(mut velocity, transform)| {
        if transform.translation.y < -0.0 && velocity.y < 0.0 {
            velocity.y = flap_force.0 * 0.5;
        }
    });
}

fn animate_velocity(
    mut query: Query<(&mut Transform, &Velocity), With<Player>>,
    game_state: Res<State<GameState>>,
    time: Res<Time>,
) {
    if *game_state == GameState::Waiting {
        return;
    }

    query.for_each_mut(|(mut transform, velocity)| {
        let multiplier = if velocity.y > 0.0 { 10.0 } else { 3.0 };
        transform.rotate_local_z(velocity.y.to_radians() * multiplier * time.delta_seconds());
        let mut euler_angles = transform.rotation.to_euler(EulerRot::XYZ);
        euler_angles.2 = euler_angles
            .2
            .clamp(-90f32.to_radians(), 20f32.to_radians());
        transform.rotation = Quat::from_euler(
            EulerRot::XYZ,
            euler_angles.0,
            euler_angles.1,
            euler_angles.2,
        );
    });
}

fn collisions(
    mut commands: Commands,
    mut query: Query<(&mut Transform, &mut Velocity, &mut GravityScale, &Collider, Entity), With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut collision_events: EventReader<CollisionEvent>,
    mut game_score: ResMut<GameScore>,
    mut shake_query: Query<&mut Shake2d>,
    game_assets: Res<GameAssets>,
    pipe_query: Query<(&GlobalTransform, &Collider), With<Pipe>>,
    pipe_area_query: Query<Entity, (With<PipeArea>, Without<Pipe>)>,
    game_state: Res<State<GameState>>,
    game_boundaries: Res<GameBoundaries>,
) {
    query.for_each_mut(
        |(mut transform, mut velocity, mut gravity_scale, collider, entity)| {
            // ground collision
            let mut shake = shake_query.single_mut();
            if transform.translation.y < game_boundaries.min.y {
                if *game_state != GameState::Dead {
                    println!("IMPACT VELOCITY: {:?}", **velocity);
                    let velocity_multiplier = ((500.0 - velocity.length()) / 300.0).clamp(0.5, 1.5);
                    println!("VALUE: {}", velocity_multiplier);
                    **velocity = Vec2::ZERO;
                    **gravity_scale = 0.0;
                    transform.translation.y = game_boundaries.min.y;

                    shake.trauma = (1.0 - (velocity_multiplier - 0.5)) * 0.4;
                    commands.entity(entity).remove::<Handle<Animation>>();

                    commands.spawn(AudioSourceBundle {
                        source: game_assets.hit_audio.clone(),
                        settings: PlaybackSettings {
                            mode: PlaybackMode::Remove,
                            volume: bevy::audio::Volume::Absolute(VolumeLevel::new(0.1)),
                            speed: velocity_multiplier,
                            ..default()
                        },
                    });

                    next_state.set(GameState::Dead);
                }
            }

            // pipe collison
            for event in collision_events.read() {
                if event.entity_a == entity {
                    if let Ok((pipe_transform, pipe_collider)) = pipe_query.get(event.entity_b) {
                        if *game_state != GameState::Stopped && *game_state != GameState::Dead {
                            shake.trauma = 0.25;
                            commands.entity(entity).remove::<Handle<Animation>>();

                            commands.spawn(AudioSourceBundle {
                                source: game_assets.hit_audio.clone(),
                                settings: PlaybackSettings {
                                    mode: PlaybackMode::Remove,
                                    volume: bevy::audio::Volume::Absolute(VolumeLevel::new(0.1)),
                                    ..default()
                                },
                            });

                            next_state.set(GameState::Stopped);
                        }

                        let overlap_y = (collider.size.y / 2.0 + pipe_collider.size.y / 2.0) - (pipe_transform.translation().y - transform.translation.y).abs();
                        let translation_vector = match event.collision {
                            Collision::Top => Vec3::new(0.0, overlap_y, 0.0),
                            Collision::Bottom => Vec3::new(0.0, -overlap_y, 0.0),
                            Collision::Inside => Vec3::new(0.0, overlap_y, 0.0),
                            _ => Vec3::ZERO,
                        };
                        println!("{:?}", event.collision);
                        transform.translation += translation_vector;
                        println!("TRANSLATED: {translation_vector}");
                    }
                    
                    // score collision
                    if pipe_area_query.contains(event.entity_b) {
                        **game_score += 1;

                        commands.entity(event.entity_b).despawn();

                        commands.spawn(AudioSourceBundle {
                            source: game_assets.point_audio.clone(),
                            settings: PlaybackSettings {
                                mode: PlaybackMode::Remove,
                                volume: bevy::audio::Volume::Absolute(VolumeLevel::new(0.1)),
                                ..default()
                            },
                        });
                    }
                }
            }
        },
    );
}
