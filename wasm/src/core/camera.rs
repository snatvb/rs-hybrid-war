use bevy::prelude::*;

pub struct MainCamera;
pub struct CursorPosition {
    pub world: Vec4,
    pub position: Vec2,
}

impl Default for CursorPosition {
    fn default() -> Self {
        Self {
            world: Default::default(),
            position: Default::default(),
        }
    }
}

pub fn setup_main_camera(mut commands: Commands) {
    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d())
        .insert(CursorPosition::default())
        .insert(MainCamera);
    crate::logger::log!("Added camera!");
}

pub fn cursor_system(
    windows: Res<Windows>,
    mut query: Query<(&Transform, &mut CursorPosition), With<MainCamera>>,
) {
    let window = windows.get_primary().unwrap();

    if let Some(cursor_position) = window.cursor_position() {
        let size = Vec2::new(window.width() as f32, window.height() as f32);
        let (camera_transform, mut camera_cursor_pos) = query.single_mut().unwrap();
        let orthographic_pos = cursor_position - size / 2.0;

        let world_pos =
            camera_transform.compute_matrix() * orthographic_pos.extend(0.0).extend(1.0);
        camera_cursor_pos.world = world_pos;
        camera_cursor_pos.position = cursor_position;
    }
}
