use std::ops::{Deref, DerefMut};

use bevy::{prelude::*, sprite::collide_aabb::collide};

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

#[derive(Resource, Default, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

// #[derive(Component, Default)]
// pub struct Velocity(Vec2);

impl From<Vec2> for Velocity {
    fn from(value: Vec2) -> Self {
        Velocity(value)
    }
}

#[derive(Component)]
struct Collider {
    size: Vec2,
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
            .insert_resource(Gravity::from(Vec2::new(0.0, -250.0)))
            .insert_resource(Velocity::default())
            .add_systems(Update, (apply_gravity).chain())
            .add_systems(PostUpdate, check_collisions);
    }
}

fn apply_gravity(mut velocity: ResMut<Velocity>, gravity: Res<Gravity>, time: Res<Time>) {
    velocity.0 += gravity.0 * time.delta_seconds();
}

fn apply_velocity(mut query: Query<&mut Transform>, velocity: Res<Velocity>, time: Res<Time>) {
    query.for_each_mut(|mut transform| {
        transform.translation += velocity.0.extend(0.0) * time.delta_seconds();
    });
}

fn check_collisions(
    collider_query: Query<(Entity, &Collider, &Transform)>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let colliders = collider_query.iter().collect::<Vec<_>>();

    for (i, (entity_a, collider_a, transform_a)) in colliders.iter().enumerate() {
        for (entity_b, collider_b, transform_b) in colliders.iter().skip(i + 1) {
            let collision = collide(
                transform_a.translation,
                collider_a.size,
                transform_b.translation,
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
