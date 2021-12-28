use std::f32::consts::FRAC_PI_2;
use bevy::prelude::*;

mod animation;
pub use animation::*;

#[derive(Default, Component)]
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

#[derive(Component)]
pub struct MainCamera;

#[derive(Component, Debug, Default, Clone, Copy)]
pub struct CursorPosition {
    pub world: Vec4,
    pub position: Vec2,
}
