#![allow(unused)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate enum_dispatch;
extern crate benimator;
extern crate bevy;

#[macro_use]
mod common;
mod animations;
mod components;
mod core;
mod input;
mod logger;
mod materials;
mod player;
mod shells;
mod sprites;

use benimator::*;
use bevy::{
    prelude::*,
    window::{WindowMode, WindowResizeConstraints},
};
use materials::Materials;
use std::time::Duration;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run() {
    logger::log!("Running game...");
    let mut app = App::build();

    // --- plugins
    app.add_plugins_with(DefaultPlugins, |group| {
        group.add_before::<bevy::asset::AssetPlugin, _>(bevy_web_asset::WebAssetPlugin)
    })
    .add_plugin(AnimationPlugin)
    .add_plugin(bevy_webgl2::WebGL2Plugin)
    .add_plugin(core::CorePlugin)
    .add_plugin(shells::ShellsPlugin);

    // --- resources
    app.insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)));
    let window = WindowDescriptor {
        title: "UNTITLED".to_string(),
        width: 1024.0,
        height: 768.0,
        resize_constraints: WindowResizeConstraints {
            min_width: 800.0,
            min_height: 600.0,
            max_width: f32::MAX,
            max_height: f32::MAX,
        },
        mode: WindowMode::BorderlessFullscreen,
        resizable: false,
        ..Default::default()
    };
    app.insert_resource(window);

    // --- start up systems
    app.add_startup_system(setup.system()).add_startup_stage(
        "game_setup_actors",
        SystemStage::single(player::spawn.system()),
    );

    // --- systems
    app.add_system(player::movement.system())
        .add_system(player::fire.system())
        .add_system(player::rotate_by_cursor.system())
        .add_system(animations::change.system());

    app.run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<Texture>>,
    mut windows: ResMut<Windows>,
) {
    commands.insert_resource(Materials {
        player: asset_server.load(sprites::PLAYER.path),
    });
}
