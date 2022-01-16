use core::prelude::*;

fn setup_room(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("images/environment/room.png"),
        ..Default::default()
    });
}

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // let texture_handle = asset_server.load("images/player/run.png");
    // let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 4, 4);
    let texture_handle = asset_server.load("images/player/idle.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 4, 3);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            ..Default::default()
        })
        .insert(AnimationSprite {
            frames: 10,
            stage: AnimationSpriteStage::Play,
            duration: Timer::from_seconds(0.16, true),
            ..Default::default()
        });
}

fn main() {
    App::new()
        .add_plugin(core::CorePlugin)
        .add_startup_system(setup_room)
        .add_startup_system(setup_player)
        .run();
}
