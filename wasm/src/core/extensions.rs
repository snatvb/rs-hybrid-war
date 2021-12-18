use bevy::prelude::*;

pub struct LookAt(pub Vec2);

pub fn look_at(mut query: Query<(&mut Transform, &LookAt)>) {
    for (mut transform, target) in query.iter_mut() {
        // TODO: Add handling rotation by struct (look at the top)
    }
}
