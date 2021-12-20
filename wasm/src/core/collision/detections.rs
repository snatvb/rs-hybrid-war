use super::components::*;
use bevy::prelude::*;

// mod angle_sides {
//     use super::super::super::math::deg_to_rad;
//     use std::ops::Range;

//     lazy_static! {
//         pub static ref TOP: Range<f32> = deg_to_rad(315.0)..deg_to_rad(45.0);
//         pub static ref RIGHT: Range<f32> = deg_to_rad(45.0)..deg_to_rad(135.0);
//         pub static ref BOTTOM: Range<f32> = deg_to_rad(135.0)..deg_to_rad(225.0);
//         pub static ref LEFT: Range<f32> = deg_to_rad(225.0)..deg_to_rad(315.0);
//     }
// }

pub fn aabb_vs_aabb(
    a_pos: Vec2,
    b_pos: Vec2,
    a_collider: &Rectangle,
    b_collider: &Rectangle,
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

pub fn circle_vs_circle(
    a_pos: Vec2,
    b_pos: Vec2,
    a_collider: &Circle,
    b_collider: &Circle,
) -> Option<Hit> {
    let distance = a_collider
        .in_world_position(a_pos)
        .distance(b_collider.in_world_position(b_pos));
    let minimal_distance = a_collider.radius() + b_collider.radius();
    if distance < minimal_distance {
        Some(Hit::CircleCircle(a_pos - b_pos))
    } else {
        None
    }
}
