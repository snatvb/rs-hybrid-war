use super::detections::detect_aabb_vs_aabb;
use bevy::prelude::*;

pub enum CollisionSide {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(From)]
pub enum Hit {
    #[allow(non_camel_case_types)]
    AABB_AABB(CollisionSide),
}

#[derive(Default)]
pub struct Rectangle(pub Vec2, pub Vec2);

impl Rectangle {
    pub fn get_min_max(&self, position: &Vec2) -> (Vec2, Vec2) {
        (*position + self.0, *position + self.1)
    }

    pub fn size(&self) -> (f32, f32) {
        (
            self.0.x.abs() + self.1.x.abs(),
            self.0.y.abs() + self.1.y.abs(),
        )
    }

    pub fn from_size_vec(size: Vec2) -> Self {
        let half = size / 2.0;

        Self(-half, half)
    }
}

impl Collide for Rectangle {
    fn collide(&self, position: Vec2, b_collider: &Collider, b_position: Vec2) -> Option<Hit> {
        match b_collider {
            Collider::Rectangle(b_collider) => {
                detect_aabb_vs_aabb(position, b_position, self, b_collider)
            }
            _ => None,
        }
    }
}

#[derive(Default)]
pub struct Circle(pub Vec2, f32);

impl Collide for Circle {
    fn collide(&self, position: Vec2, b_collider: &Collider, b_position: Vec2) -> Option<Hit> {
        match b_collider {
            _ => None,
        }
    }
}

#[enum_dispatch]
pub enum Collider {
    Rectangle(Rectangle),
    Circle(Circle),
}

#[enum_dispatch(Collider)]
pub trait Collide {
    fn collide(&self, position: Vec2, b_collider: &Collider, b_position: Vec2) -> Option<Hit>;
}

pub struct RigidBody {
    pub collider: Collider,
    pub solid: bool,
}
