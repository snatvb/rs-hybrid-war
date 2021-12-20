use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use super::{Collided, Collider, RigidBody};

pub struct DebugVisualize;
pub struct DebugVisualizer(Entity);

impl DebugVisualizer {
    pub fn target_entity(&self) -> Entity {
        self.0
    }
}

pub fn visualize_detect(
    mut commands: Commands,
    query: Query<(Entity, &RigidBody, &Transform), Without<DebugVisualize>>,
) {
    for (entity, rb, transform) in query.iter() {
        match &rb.collider {
            Collider::Rectangle(rect) => {
                let (width, height) = rect.size();
                let shape = shapes::Rectangle {
                    width,
                    height,
                    ..Default::default()
                };
                commands.entity(entity).insert(DebugVisualize);
                commands
                    .spawn_bundle(GeometryBuilder::build_as(
                        &shape,
                        ShapeColors::outlined(Color::GREEN, Color::BLACK),
                        DrawMode::Stroke(StrokeOptions::default().with_line_width(2.0)),
                        transform.clone(),
                    ))
                    .insert(DebugVisualizer(entity));
            }
            Collider::Circle(circle) => {
                let shape = shapes::Circle {
                    radius: circle.radius(),
                    center: circle.offset(),
                };
                commands.entity(entity).insert(DebugVisualize);
                commands
                    .spawn_bundle(GeometryBuilder::build_as(
                        &shape,
                        ShapeColors::outlined(Color::GREEN, Color::BLACK),
                        DrawMode::Stroke(StrokeOptions::default().with_line_width(2.0)),
                        transform.clone(),
                    ))
                    .insert(DebugVisualizer(entity));
            }
            _ => (),
        }
    }
}

pub fn visualize_sync_position(
    mut commands: Commands,
    mut query: Query<(Entity, &DebugVisualizer, &mut Transform)>,
    entities: Query<&Transform, (With<DebugVisualize>, Without<DebugVisualizer>)>,
) {
    for (entity, visualizer, mut transform) in query.iter_mut() {
        if let Ok(target_transform) = entities.get(visualizer.target_entity()) {
            transform.translation = target_transform.translation;
        } else {
            commands.entity(entity).despawn();
        }
    }
}

pub fn log_collide(mut collide_events: EventReader<Collided>) {
    for collided in collide_events.iter() {
        crate::logger::log!("Collided {:?}", collided);
    }
}
