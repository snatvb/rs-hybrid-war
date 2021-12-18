use std::ops::Range;

use bevy::prelude::*;

use crate::components::Velocity;

const FRICTION_PRECISION: Range<f32> = -0.1..0.1;

pub fn movement(mut query: Query<(&mut Transform, &Velocity)>) {
    if let Ok((mut transform, velocity)) = query.single_mut() {
        transform.translation.x += velocity.x;
        transform.translation.y += velocity.y;
    }
}

pub fn friction(mut query: Query<(&mut Velocity)>) {
    if let Ok((mut velocity)) = query.single_mut() {
        velocity.x -= velocity.x * 0.3;
        velocity.y -= velocity.y * 0.3;

        if FRICTION_PRECISION.contains(&velocity.x) {
            velocity.x = 0.0;
        }
        if FRICTION_PRECISION.contains(&velocity.y) {
            velocity.y = 0.0;
        }
    }
}
