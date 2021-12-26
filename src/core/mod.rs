use bevy::{prelude::*, diagnostic::FrameTimeDiagnosticsPlugin};
use bevy_prototype_lyon::plugin::ShapePlugin;

mod components;
mod systems;
mod monitoring;

pub struct Core;

impl Plugin for Core {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins);
        app.add_plugin(FrameTimeDiagnosticsPlugin::default());
        app.add_plugin(ShapePlugin);

        app.add_startup_system(setup.system());
        app.add_startup_system(monitoring::setup.system());

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
