use bevy::prelude::*;

#[derive(From)]
pub enum Collider {
    Box(Vec2, Vec2),
    Circle(u32),
}

pub struct RigidBody {
    pub collider: Collider,
    pub solid: bool,
    pub scale: f32,
}

pub fn detection(query: Query<(Entity, &RigidBody, &Transform)>) {
    for (a_entity, a_rigid_body, a_transform) in query.iter() {
        for (b_entity, b_rigid_body, b_transform) in query
            .iter()
            .filter(|(b_entity, _, _)| !b_entity.eq(&a_entity))
        {
            let a_collider_lt = Vec2::new(-1.0, -1.0);
            let a_collider_rb = Vec2::new(1.0, 1.0);
            let b_collider_lt = Vec2::new(-1.0, -1.0);
            let b_collider_rb = Vec2::new(1.0, 1.0);

            let a_min = a_transform.translation.truncate() + a_collider_lt;
            let a_max = a_transform.translation.truncate() + a_collider_rb;

            let b_min = b_transform.translation.truncate() + b_collider_lt;
            let b_max = b_transform.translation.truncate() + b_collider_rb;

            if a_min.x < b_max.x && a_max.x > b_min.x && a_min.y < b_max.y && a_max.y > b_min.y {}
        }
    }
}
