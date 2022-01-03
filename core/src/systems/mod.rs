use super::components::*;
use bevy::prelude::*;
use console::{ConsoleCommandEvent, ConsoleLogEvent, LogKind};

pub mod animation;

pub fn look_at(mut query: Query<(&mut Transform, &LookAt), (Changed<Transform>, Changed<LookAt>)>) {
    for (mut transform, target) in query.iter_mut() {
        transform.rotation = target.look(transform.translation.truncate());
    }
}

pub fn setup_main_camera(mut commands: Commands) {
    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d())
        .insert(CursorPosition::default())
        .insert(MainCamera::default());
}

pub fn camera_zoom(mut query: Query<(&MainCamera, &mut OrthographicProjection)>) {
    for (main_camera, mut projection) in query.iter_mut() {
        projection.scale = main_camera.zoom;
    }
}

pub fn debug_set_scale(mut query: Query<&mut MainCamera>, mut cmd_events: EventReader<ConsoleCommandEvent>, mut events: EventWriter<ConsoleLogEvent>) {
    for cmd_event in cmd_events.iter().filter(|e| e.cmd == "set_camera_zoom") {
        for mut main_camera in query.iter_mut() {
            if let Some(scale) = cmd_event.args.first().and_then(|s| s.parse::<f32>().ok()).map(|s| s.clamp(0.01, 5.0)) {
                main_camera.zoom = scale;
                events.send(ConsoleLogEvent::new(format!("Success install: {}", scale), LogKind::Info));
            } else {
                events.send(ConsoleLogEvent::new("Wrong camera zoom command", LogKind::Warning));
            }
        }
    }
}

pub fn cursor_system(
    windows: Res<Windows>,
    mut query: Query<(&Transform, &mut CursorPosition), With<MainCamera>>,
) {
    let window = windows.get_primary().unwrap();

    if let Some(cursor_position) = window.cursor_position() {
        let size = Vec2::new(window.width() as f32, window.height() as f32);
        let (camera_transform, mut camera_cursor_pos) = query.single_mut();
        let orthographic_pos = cursor_position - size / 2.0;

        let world_pos =
            camera_transform.compute_matrix() * orthographic_pos.extend(0.0).extend(1.0);
        camera_cursor_pos.world = world_pos;
        camera_cursor_pos.position = cursor_position;
    }
}
