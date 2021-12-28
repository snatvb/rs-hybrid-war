use bevy::{prelude::*, diagnostic::FrameTimeDiagnosticsPlugin};
// use bevy_prototype_lyon::plugin::ShapePlugin;

pub mod components;
pub mod prelude;
mod systems;
mod monitoring;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins);
        // app.add_plugin(ShapePlugin);
        app.add_plugin(console::ConsolePlugin);
        app.add_plugin(FrameTimeDiagnosticsPlugin::default());

        app.add_startup_system(setup.system());
        app.add_startup_system(monitoring::setup.system());
        app.add_startup_system(systems::setup_main_camera.system());

        app.add_system_to_stage(CoreStage::First, systems::animation::animate.system());
        app.add_system(systems::look_at.system());
        app.add_system(systems::cursor_system.system());
        app.add_system(monitoring::fps_update_system.system());
        app.add_system(monitoring::ms_update_system.system());
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
}
