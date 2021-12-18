use bevy::prelude::*;

pub mod camera;
pub mod extensions;
pub mod math;
pub mod physics;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut AppBuilder) {
        // start up
        app.add_startup_system(camera::setup_main_camera.system());
        // systems
        app.add_system(camera::cursor_system.system())
            .add_system(physics::movement.system())
            .add_system(physics::friction.system())
            .add_system(extensions::look_at.system());
    }
}
