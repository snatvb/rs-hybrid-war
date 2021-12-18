use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;

#[derive(Default)]
pub struct LookAt {
    pub target: Vec2,
    pub angle_offset: f32, // in radian
}

impl LookAt {
    pub fn new(target: Vec2) -> Self {
        Self {
            target,
            ..Default::default()
        }
    }

    pub fn look(&self, current_position: Vec2) -> Quat {
        let direction = self.target - current_position;
        let angle = direction.angle_between(Vec2::new(1.0, 0.0));
        Quat::from_rotation_z(FRAC_PI_2 - angle + self.angle_offset)
    }
}

pub fn look_at(mut query: Query<(&mut Transform, &LookAt), (Changed<Transform>, Changed<LookAt>)>) {
    for (mut transform, target) in query.iter_mut() {
        transform.rotation = target.look(transform.translation.truncate());
    }
}
