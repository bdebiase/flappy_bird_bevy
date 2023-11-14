use std::ops::{Deref, DerefMut};

use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

#[derive(Resource)]
pub struct Gravity(Vec2);

impl From<Vec2> for Gravity {
    fn from(vec: Vec2) -> Self {
        Self(vec)
    }
}

impl Deref for Gravity {
    type Target = Vec2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Gravity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct GravityMultiplier(pub f32);

impl Default for GravityMultiplier {
    fn default() -> Self {
        Self(1.0)
    }
}

#[derive(Component, Default, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

impl From<Vec2> for Velocity {
    fn from(value: Vec2) -> Self {
        Velocity(value)
    }
}

#[derive(Component)]
pub struct Collider {
    pub size: Vec2,
}

impl From<Vec2> for Collider {
    fn from(value: Vec2) -> Self {
        Self { size: value }
    }
}

#[derive(Event)]
pub struct CollisionEvent {
    pub entity_a: Entity,
    pub entity_b: Entity,
}

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>()
            .insert_resource(Gravity::from(Vec2::new(0.0, -100.0)))
            .add_systems(PreUpdate, check_collisions)
            .add_systems(Update, (apply_gravity, apply_velocity));
    }
}

fn apply_gravity(
    mut query: Query<(&mut Velocity, Option<&GravityMultiplier>)>,
    gravity: Res<Gravity>,
    time: Res<Time>,
) {
    query.for_each_mut(|(mut velocity, opt_multiplier)| {
        let mut multiplier = 1.0;
        if let Some(value) = opt_multiplier {
            multiplier = **value;
        }
        **velocity += **gravity * multiplier * time.delta_seconds();
    });
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    query.for_each_mut(|(mut transform, velocity)| {
        transform.translation += velocity.extend(0.0) * time.delta_seconds();
    });
}

fn check_collisions(
    mut collision_events: EventWriter<CollisionEvent>,
    collider_query: Query<(Entity, &Collider, &GlobalTransform)>,
) {
    let colliders = collider_query.iter().collect::<Vec<_>>();

    for (i, (entity_a, collider_a, transform_a)) in colliders.iter().enumerate() {
        for (entity_b, collider_b, transform_b) in colliders.iter().skip(i + 1) {
            let collision = collide(
                transform_a.translation(),
                collider_a.size,
                transform_b.translation(),
                collider_b.size,
            );

            if let Some(_collision) = collision {
                collision_events.send(CollisionEvent {
                    entity_a: *entity_a,
                    entity_b: *entity_b,
                });
            }
        }
    }
}

fn draw_debug(mut gizmos: Gizmos, query: Query<(&GlobalTransform, &Collider)>) {
    query.for_each(|(transform, collider)| {
        gizmos.line_2d(
            transform.translation().truncate()
                - Vec2::X * collider.size * 0.5
                - Vec2::Y * collider.size * 0.5,
            transform.translation().truncate() - Vec2::X * collider.size * 0.5
                + Vec2::Y * collider.size * 0.5,
            Color::MIDNIGHT_BLUE,
        );
        gizmos.line_2d(
            transform.translation().truncate() + Vec2::X * collider.size * 0.5
                - Vec2::Y * collider.size * 0.5,
            transform.translation().truncate()
                + Vec2::X * collider.size * 0.5
                + Vec2::Y * collider.size * 0.5,
            Color::MIDNIGHT_BLUE,
        );
        gizmos.line_2d(
            transform.translation().truncate()
                - Vec2::X * collider.size * 0.5
                - Vec2::Y * collider.size * 0.5,
            transform.translation().truncate() + Vec2::X * collider.size * 0.5
                - Vec2::Y * collider.size * 0.5,
            Color::MIDNIGHT_BLUE,
        );
        gizmos.line_2d(
            transform.translation().truncate() - Vec2::X * collider.size * 0.5
                + Vec2::Y * collider.size * 0.5,
            transform.translation().truncate()
                + Vec2::X * collider.size * 0.5
                + Vec2::Y * collider.size * 0.5,
            Color::MIDNIGHT_BLUE,
        );
    });
}
