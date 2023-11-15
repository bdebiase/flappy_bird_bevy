use bevy::prelude::*;

pub fn despawn<T: Component>(query: Query<Entity, With<T>>, mut commands: Commands) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}
