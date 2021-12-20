use std::collections::HashMap;

use bevy::prelude::*;

mod components;
mod debug;
mod detections;

pub use components::*;

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

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<Collided>().add_system(detection.system());
        app.add_system(debug::visualize_detect.system());
        app.add_system(debug::visualize_sync_position.system());
    }
}
