use core::prelude::*;

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("images/environment/room.png"),
        ..Default::default()
    });
}

fn main() {
    App::new()
        .add_plugin(core::CorePlugin)
        .add_startup_system(startup)
        .run();
}