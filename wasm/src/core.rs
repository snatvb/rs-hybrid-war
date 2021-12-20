use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, window::WindowResized};
use bevy_prototype_lyon::plugin::ShapePlugin;

pub mod camera;
mod canvas;
pub mod collision;
pub mod extensions;
pub mod math;
pub mod monitoring;
pub mod physics;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut AppBuilder) {
        // plugins
        app.add_plugin(FrameTimeDiagnosticsPlugin::default());
        app.add_plugin(ShapePlugin);
        app.add_plugin(collision::CollisionPlugin);
        // resources
        app.insert_resource(canvas::LastSize::default())
            .insert_resource(Msaa { samples: 4 });
        // start up
        app.add_startup_system(setup.system())
            .add_startup_system(monitoring::setup.system())
            .add_startup_system(camera::setup_main_camera.system());
        // systems
        app.add_system(camera::cursor_system.system())
            .add_system(physics::movement.system())
            .add_system(physics::friction.system())
            .add_system(extensions::look_at.system())
            .add_system(monitoring::fps_update_system.system())
            .add_system(monitoring::ms_update_system.system())
            .add_system(canvas::resize_watch.system());
    }
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
}
