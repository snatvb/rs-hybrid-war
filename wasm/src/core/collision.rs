use std::collections::HashMap;

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

pub fn detect_aabb_vs_aabb(
    a_pos: Vec2,
    b_pos: Vec2,
    a_collider: &Box,
    b_collider: &Box,
) -> Option<Hit> {
    let (a_min, a_max) = a_collider.get_min_max(&a_pos);
    let (b_min, b_max) = b_collider.get_min_max(&b_pos);

    // check to see if the two rectangles are intersecting
    if a_min.x < b_max.x && a_max.x > b_min.x && a_min.y < b_max.y && a_max.y > b_min.y {
        // check to see if we hit on the left or right side
        let (x_collision, x_depth) = if a_min.x < b_min.x && a_max.x > b_min.x && a_max.x < b_max.x
        {
            (Some(CollisionSide::Left), b_min.x - a_max.x)
        } else if a_min.x > b_min.x && a_min.x < b_max.x && a_max.x > b_max.x {
            (Some(CollisionSide::Right), a_min.x - b_max.x)
        } else {
            (None, 0.0)
        };

        // check to see if we hit on the top or bottom side
        let (y_collision, y_depth) = if a_min.y < b_min.y && a_max.y > b_min.y && a_max.y < b_max.y
        {
            (Some(CollisionSide::Bottom), b_min.y - a_max.y)
        } else if a_min.y > b_min.y && a_min.y < b_max.y && a_max.y > b_max.y {
            (Some(CollisionSide::Top), a_min.y - b_max.y)
        } else {
            (None, 0.0)
        };

        // if we had an "x" and a "y" collision, pick the "primary" side using penetration depth
        match (x_collision, y_collision) {
            (Some(x_collision), Some(y_collision)) => {
                if y_depth.abs() < x_depth.abs() {
                    Some(y_collision)
                } else {
                    Some(x_collision)
                }
            }
            (Some(x_collision), None) => Some(x_collision),
            (None, Some(y_collision)) => Some(y_collision),
            (None, None) => None,
        }
        .map(Hit::from)
    } else {
        None
    }
}

#[derive(Default)]
pub struct Box(pub Vec2, pub Vec2);

impl Box {
    fn get_min_max(&self, position: &Vec2) -> (Vec2, Vec2) {
        (*position + self.0, *position + self.1)
    }
}

impl Collide for Box {
    fn collide(&self, position: Vec2, b_collider: &Collider, b_position: Vec2) -> Option<Hit> {
        match b_collider {
            Collider::Box(b_collider) => {
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
    Box(Box),
    Circle(Circle),
}

#[enum_dispatch(Collider)]
trait Collide {
    fn collide(&self, position: Vec2, b_collider: &Collider, b_position: Vec2) -> Option<Hit>;
}

pub struct RigidBody {
    pub collider: Collider,
    pub solid: bool,
}

pub struct Collided {
    pub hit: Hit,
    pub a_entity: Entity,
    pub b_entity: Entity,
}

pub fn detection(
    query: Query<(Entity, &RigidBody, &Transform)>,
    mut events: EventWriter<Collided>,
) {
    let mut checked_cache: HashMap<(Entity, Entity), bool> = HashMap::new();
    for (a_entity, a_rigid_body, a_transform) in query.iter() {
        checked_cache.insert((a_entity.clone(), a_entity.clone()), true);
        for (b_entity, b_rigid_body, b_transform) in query.iter() {
            if checked_cache.get(&(a_entity, b_entity)).is_none() {
                checked_cache.insert((a_entity.clone(), b_entity.clone()), true);
            } else {
                continue;
            }

            if let Some(hit) = a_rigid_body.collider.collide(
                a_transform.translation.truncate(),
                &b_rigid_body.collider,
                b_transform.translation.truncate(),
            ) {
                events.send(Collided {
                    hit,
                    a_entity,
                    b_entity,
                });
            }
        }
    }
}
